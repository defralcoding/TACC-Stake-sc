#![no_std]
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const YEAR_IN_SECONDS: u64 = 60 * 60 * 24 * 365;
const PERCENTAGE: u32 = 100;

const UNBOND_DAYS: u64 = 6;
//const DAY_IN_SECONDS: u64 = 60 * 60 * 24;
const DAY_IN_SECONDS: u64 = 60 * 2; // 2 minutes for testing

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct UnstakingPosition {
    pub nonce: u64,
    pub deadline: u64,
}

#[elrond_wasm::contract]
pub trait TiredClub {
    #[init]
    fn init(
        &self
    ) {
    }

    #[payable("*")]
    #[endpoint(stakeFirst)]
    fn stake_first(&self,
        #[payment_token] payment_token: EgldOrEsdtTokenIdentifier,
        #[payment_nonce] nonce: u64) {
        
        let caller = self.blockchain().get_caller();
        let first_collection_token = self.first_collection_token().get();

        require!(payment_token == first_collection_token, "Invalid NFT input, {} expected", first_collection_token);

        self.users_staked_first_collection().insert(caller.clone());
        self.user_staked_first_collection(&caller).insert(nonce);
    }

    #[payable("*")]
    #[endpoint(stakeSecond)]
    fn stake_second(&self,
        #[payment_token] payment_token: EgldOrEsdtTokenIdentifier,
        #[payment_nonce] nonce: u64) {
        
        let caller = self.blockchain().get_caller();
        let second_collection_token = self.second_collection_token().get();

        require!(payment_token == second_collection_token, "Invalid NFT input, {} expected", second_collection_token);

        self.users_staked_second_collection().insert(caller.clone());
        self.user_staked_second_collection(&caller).insert(nonce);
    }
    
    #[endpoint(unstakeFirst)]
    fn unstake_first(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        let is_nonce_staked = self.user_staked_first_collection(&caller).swap_remove(&nonce);
        require!(!is_nonce_staked, "NFT was not staked");

        let unstaking_position = UnstakingPosition {
            nonce,
            deadline: self.blockchain().get_block_timestamp() + UNBOND_DAYS * DAY_IN_SECONDS,
        };

        self.user_unstaked_first_collection(&caller).insert(unstaking_position);
        self.number_unstaked_first_collection().update(|number| *number += 1);
    }

/*
    #[endpoint(unstake)]
    fn unstake(&self, unstake_amount: BigUint) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let staked_amount = self.staked_amount(&caller).get();
        let first_collection_token = self.first_collection_token().get();

        require!(self.lock_time(&caller).get() < current_timestamp , "Lock time did not passed");
        require!(unstake_amount <= staked_amount, "Unstake amount is greater than your staked amount");

        self.staked_amount(&caller).update(|unstaked| *unstaked -= &unstake_amount);
        self.staked_all().update(|unstaked| *unstaked -= &unstake_amount);
        self.send().direct(&caller, &first_collection_token, 0u64, &unstake_amount);
        if self.staked_amount(&caller).get() == BigUint::zero() {
            self.user_staked().swap_remove(&caller);
        }
    }

    fn claim_rewards_to_storage(&self,caller:ManagedAddress){
        let current_timestamp = self.blockchain().get_block_timestamp();
        let rewards = self.calculated_rewards();
        self.storage_rewards(&caller).update(|reward| *reward +=rewards);
        self.reset_timestamp(&self.blockchain().get_caller()).set(current_timestamp);
    }
*/
  

    /*
        FIRST COLLECTION - TACC
    */

    #[storage_mapper("stakingFirstCollectionIdentifier")]
    fn first_collection_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getUsersStakedFirstCollection)]
    #[storage_mapper("usersStakedFirstCollection")]
    fn users_staked_first_collection(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getUserStakedFirstCollection)]
    #[storage_mapper("userStakedFirstCollection")]
    fn user_staked_first_collection(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getUserUnstakedFirstCollection)]
    #[storage_mapper("userUnstakedFirstCollection")]
    fn user_unstaked_first_collection(&self, user: &ManagedAddress) -> UnorderedSetMapper<UnstakingPosition>;

    #[view(getNumberUnstakedFirstCollection)]
    #[storage_mapper("numberUnstakedFirstCollection")]
    fn number_unstaked_first_collection(&self) -> SingleValueMapper<u32>;

    /*
        SECOND COLLECTION - TASC
    */

    #[storage_mapper("stakingSecondCollectionIdentifier")]
    fn second_collection_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getUsersStakedSecondCollection)]
    #[storage_mapper("usersStakedSecondCollection")]
    fn users_staked_second_collection(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getUserStakedSecondCollection)]
    #[storage_mapper("userStakedSecondCollection")]
    fn user_staked_second_collection(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;


}
