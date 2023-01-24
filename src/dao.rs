elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const HOUR_IN_SECONDS: u64 = 60 * 60;

const TACC_TIER_NFTS: u16 = 15;
const TASC_TIER_NFTS: u16 = 10;
const COUNCIL_TIER_NFTS: u16 = 3;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct Proposal<M: ManagedTypeApi> {
    pub proposal_id: ManagedBuffer<M>,
    pub proposal_number: u16,
    pub proposal_title: ManagedBuffer<M>,
    pub proposal_content: ManagedBuffer<M>,
    pub creation_timestamp: u64,
    pub deadline_timestamp: u64,

    pub total_voting_power: u16,
    pub total_yes: u16,
    pub total_no: u16,
}

#[elrond_wasm::module]
pub trait Dao: crate::storage::Storage {
    #[endpoint(createProposal)]
    #[only_owner]
    fn create_proposal(
        &self,
        proposal_title: ManagedBuffer,
        proposal_content: ManagedBuffer,
        deadline_hours: u64,
    ) -> ManagedBuffer {
        //CREATE THE PROPOSAL
        let proposal_id = self.create_hash();
        let creation_timestamp = self.blockchain().get_block_timestamp();
        let deadline_timestamp = creation_timestamp + deadline_hours * HOUR_IN_SECONDS;

        //CALCULATE THE VOTING POWER
        let mut total_voting_power = 0u16;

        //CALCULATE THE VOTING POWER FOR TASC
        for user in self.users_staked_second_collection().iter() {
            let user_staked = self.user_staked_second_collection(&user);
            let mut user_voting_power = user_staked.len() as u16 / TASC_TIER_NFTS + 1;

            //CALCULATE COUNCIL VOTING POWER
            let n_olympians = self
                .user_number_staked_olympian_second_collection(&user)
                .get();
            //TODO legendary
            let n_legendaries = 0u32;
            let n_council = n_olympians + n_legendaries;
            if n_council > 0 {
                let council_voting_power = (n_council as u16 - 1) / COUNCIL_TIER_NFTS + 1;
                user_voting_power += council_voting_power;
            }

            self.voting_power(&proposal_id, &user)
                .set(&user_voting_power);
            total_voting_power += user_voting_power;
        }

        //CALCULATE THE VOTING POWER FOR TACC
        for user in self.users_staked_first_collection().iter() {
            let user_staked = self.user_staked_first_collection(&user);
            let user_voting_power = user_staked.len() as u16 / TACC_TIER_NFTS + 1;

            self.voting_power(&proposal_id, &user)
                .update(|user_vp| *user_vp += user_voting_power);

            total_voting_power += user_voting_power;
        }

        //STORE THE VOTING POWER TOTAL
        self.voting_power_total(&proposal_id)
            .set(&total_voting_power);

        //CREATE THE PROPOSAL STRUCT
        let proposal = Proposal {
            proposal_id: proposal_id.clone(),
            proposal_number: self.proposals().len() as u16 + 1,
            proposal_title,
            proposal_content,
            creation_timestamp,
            deadline_timestamp,
            total_voting_power,
            total_yes: 0,
            total_no: 0,
        };

        //STORE THE PROPOSAL
        self.proposal_by_id(&proposal_id).set(&proposal);
        self.proposals().push(&proposal_id);

        //RETURN THE PROPOSAL ID
        proposal_id
    }

    #[endpoint(vote)]
    fn cast_vote(&self, proposal_id: ManagedBuffer, vote: u8) {
        // get proposal and check if it exists
        let mut proposal = self.proposal_by_id(&proposal_id).get();
        //require!(proposal.is_some(), "Proposal does not exist");

        // check if the vote is within the deadline
        let now = self.blockchain().get_block_timestamp();
        require!(
            now < proposal.deadline_timestamp,
            "Voting deadline has passed"
        );

        // check if the user has already voted
        let user = self.blockchain().get_caller();
        require!(
            self.user_vote(&user, &proposal_id).get() == 0,
            "User has already voted"
        );

        // cast the vote
        let voting_power_user = self.voting_power(&proposal_id, &user).get();
        self.user_vote(&user, &proposal_id).set(&vote);
        self.voted_power(&proposal_id)
            .update(|vp| *vp += voting_power_user);
        self.vote_total(&proposal_id, vote)
            .update(|vt| *vt += voting_power_user);

        // update the proposal
        proposal.total_yes = self.vote_total(&proposal_id, 1).get();
        proposal.total_no = self.vote_total(&proposal_id, 2).get();
        self.proposal_by_id(&proposal_id).set(&proposal);
    }

    #[view(getProposalById)]
    #[storage_mapper("proposalById")]
    fn proposal_by_id(&self, proposal_id: &ManagedBuffer)
        -> SingleValueMapper<Proposal<Self::Api>>;

    #[view(getProposals)]
    #[storage_mapper("proposals")]
    fn proposals(&self) -> VecMapper<ManagedBuffer<Self::Api>>;

    #[view(getVotingPowers)]
    #[storage_mapper("votingPower")]
    fn voting_power(
        &self,
        proposal_id: &ManagedBuffer,
        user: &ManagedAddress,
    ) -> SingleValueMapper<u16>;

    #[view(getVotingPowerTotal)]
    #[storage_mapper("votingPowerTotal")]
    fn voting_power_total(&self, proposal_id: &ManagedBuffer) -> SingleValueMapper<u16>;

    #[view(getVotedPower)]
    #[storage_mapper("votedPower")]
    fn voted_power(&self, proposal_id: &ManagedBuffer) -> SingleValueMapper<u16>;

    #[view(getUserVote)]
    #[storage_mapper("userVote")]
    fn user_vote(
        &self,
        user: &ManagedAddress,
        proposal_id: &ManagedBuffer,
    ) -> SingleValueMapper<u8>;

    #[view(getVoteTotal)]
    #[storage_mapper("voteTotal")]
    fn vote_total(&self, proposal_id: &ManagedBuffer, vote: u8) -> SingleValueMapper<u16>;

    fn create_hash(&self) -> ManagedBuffer {
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        let rand_hash = rand_source.next_bytes(8);
        return rand_hash;
    }
}
