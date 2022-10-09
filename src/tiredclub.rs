#![no_std]
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const UNBOND_DAYS: u64 = 6;
const DAY_IN_SECONDS: u64 = 60 * 60 * 24;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct UnstakingPosition {
    pub nonce: u64,
    pub deadline: u64,
}

#[elrond_wasm::contract]
pub trait TiredClub: elrond_wasm_modules::dns::DnsModule {
    #[init]
    fn init(
        &self
    ) {
    }

    /*
        ***** FIRST COLLECTION - TACC *****
    */
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
    
    #[endpoint(unstakeFirst)]
    fn unstake_first(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //check if nonce is staked and remove it
        let is_nonce_staked = self.user_staked_first_collection(&caller).swap_remove(&nonce);
        require!(is_nonce_staked, "NFT was not staked");

        let unstaking_position = UnstakingPosition {
            nonce,
            deadline: self.blockchain().get_block_timestamp() + UNBOND_DAYS * DAY_IN_SECONDS,
        };

        //update storage
        self.user_unstaked_first_collection(&caller).insert(unstaking_position);
        self.number_unstaked_first_collection().update(|number| *number += 1);
    }
    
    #[endpoint(withdrawFirst)]
    fn withdraw_first(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //get unstaking position
        let user_unstaked = self.user_unstaked_first_collection(&caller);
        let mut user_unstaked_iter = user_unstaked.iter();
        let unstaking_position = user_unstaked_iter.find(|position| position.nonce == nonce);
        
        //check if NFT was already unstaked
        require!(unstaking_position.is_some(), "NFT needs to be unstaked first");
        //check if deadline is passed
        require!(unstaking_position.as_ref().unwrap().deadline < self.blockchain().get_block_timestamp(), "NFT is still locked");

        //update storage
        self.user_unstaked_first_collection(&caller).swap_remove(&unstaking_position.unwrap());
        self.number_unstaked_first_collection().update(|number| *number -= 1);

        //send NFT back to user
        let first_collection_token = self.first_collection_token().get();
        self.send().direct(&caller, &first_collection_token, nonce, &BigUint::from(1u32));
    }

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
        ***** SECOND COLLECTION - TASC *****
    */

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

    #[endpoint(unstakeSecond)]
    fn unstake_second(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //check if nonce is staked and remove it
        let is_nonce_staked = self.user_staked_second_collection(&caller).swap_remove(&nonce);
        require!(is_nonce_staked, "NFT was not staked");

        let unstaking_position = UnstakingPosition {
            nonce,
            deadline: self.blockchain().get_block_timestamp() + UNBOND_DAYS * DAY_IN_SECONDS,
        };

        //update storage
        self.user_unstaked_second_collection(&caller).insert(unstaking_position);
        self.number_unstaked_second_collection().update(|number| *number += 1);
    }
    
    #[endpoint(withdrawSecond)]
    fn withdraw_second(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //get unstaking position
        let user_unstaked = self.user_unstaked_second_collection(&caller);
        let mut user_unstaked_iter = user_unstaked.iter();
        let unstaking_position = user_unstaked_iter.find(|position| position.nonce == nonce);
        
        //check if NFT was already unstaked
        require!(unstaking_position.is_some(), "NFT needs to be unstaked first");
        //check if deadline is passed
        require!(unstaking_position.as_ref().unwrap().deadline < self.blockchain().get_block_timestamp(), "NFT is still locked");

        //update storage
        self.user_unstaked_second_collection(&caller).swap_remove(&unstaking_position.unwrap());
        self.number_unstaked_second_collection().update(|number| *number -= 1);

        //send NFT back to user
        let second_collection_token = self.second_collection_token().get();
        self.send().direct(&caller, &second_collection_token, nonce, &BigUint::from(1u32));
    }

    #[storage_mapper("stakingSecondCollectionIdentifier")]
    fn second_collection_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getUsersStakedSecondCollection)]
    #[storage_mapper("usersStakedSecondCollection")]
    fn users_staked_second_collection(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getUserStakedSecondCollection)]
    #[storage_mapper("userStakedSecondCollection")]
    fn user_staked_second_collection(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;
    
    #[view(getUserUnstakedSecondCollection)]
    #[storage_mapper("userUnstakedSecondCollection")]
    fn user_unstaked_second_collection(&self, user: &ManagedAddress) -> UnorderedSetMapper<UnstakingPosition>;

    #[view(getNumberUnstakedSecondCollection)]
    #[storage_mapper("numberUnstakedSecondCollection")]
    fn number_unstaked_second_collection(&self) -> SingleValueMapper<u32>;


}
