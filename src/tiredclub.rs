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

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct TeamMember<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub percent: u8,
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

        self.stake_first_internal(caller, nonce)
    }
    
    #[payable("*")]
    #[endpoint(stakeMultipleFirst)]
    fn stake_multiple_first(&self,
        #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>) {
        
        let caller = self.blockchain().get_caller();
        let first_collection_token = self.first_collection_token().get();

        for payment in &payments {
            let token_payment = EgldOrEsdtTokenPayment::from(payment);

            require!(token_payment.token_identifier == first_collection_token, "Invalid NFT input, {} expected", first_collection_token);

            self.stake_first_internal(caller.clone(), token_payment.token_nonce);
        }
    }

    fn stake_first_internal(&self, user: ManagedAddress, nonce: u64) {
        self.users_staked_first_collection().insert(user.clone());
        self.user_staked_first_collection(&user).insert(nonce);
        self.number_staked_first_collection().update(|number| *number += 1);
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
        self.number_staked_first_collection().update(|number| *number -= 1);
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
    
    #[endpoint(cancelUnstakeFirst)]
    fn cancel_unstake_first(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //get unstaking position
        let user_unstaked = self.user_unstaked_first_collection(&caller);
        let mut user_unstaked_iter = user_unstaked.iter();
        let unstaking_position = user_unstaked_iter.find(|position| position.nonce == nonce);
        
        //check if NFT was already unstaked
        require!(unstaking_position.is_some(), "NFT needs to be unstaked first");
        
        //update storage
        self.user_unstaked_first_collection(&caller).swap_remove(&unstaking_position.unwrap());
        self.number_unstaked_first_collection().update(|number| *number -= 1);

        //get NFT back to stake
        self.stake_first_internal(caller, nonce);
    }

    #[payable("EGLD")]
    #[endpoint(distributeRoyaltiesFirstCollection)]
    #[only_owner]
    fn distribute_royalties_first_collection(&self,
        #[payment_amount] payment_amount: BigUint) {
        self.distribute_to_tasc_stakers(&payment_amount);
    }

    #[payable("EGLD")]
    #[endpoint(distributeRoyaltiesSecondCollection)]
    #[only_owner]
    fn distribute_royalties_second_collection(&self,
        #[payment_amount] payment_amount: BigUint) {
/*
        let team_royalties_second_collection = self.team_royalties_second_collection().get();
        let team_amount = &payment_amount * &BigUint::from(team_royalties_second_collection) / &BigUint::from(100u8);
*/     
        let tasc_royalties_second_collection = self.tasc_royalties_second_collection().get();
        let tasc_amount = &payment_amount * &BigUint::from(tasc_royalties_second_collection) / &BigUint::from(100u8);
       
        let tacc_amount = payment_amount /* - &team_amount */ - &tasc_amount;
        
        //self.distribute_to_team(&team_amount);
        self.distribute_to_tasc_stakers(&tasc_amount);
        self.distribute_to_tacc_stakers(&tacc_amount);
    }

    fn distribute_to_team(&self, amount: &BigUint) {
        for team_member in self.team_addresses().iter() {
            let member_amount = amount * &BigUint::from(team_member.percent) / &BigUint::from(100u8);
            self.send().direct(&team_member.address, &EgldOrEsdtTokenIdentifier::egld(), 0, &member_amount);
        }
    }

    fn distribute_to_tacc_stakers(&self, amount: &BigUint) {
        let number_nfts_staked = self.number_staked_first_collection().get();
        let tokens_per_nft = amount / number_nfts_staked;

        for user_staked in self.users_staked_first_collection().iter() {
            let user_nfts_number = BigUint::from(self.user_staked_first_collection(&user_staked).len());
            self.user_rewards(&user_staked).update(|rewards| *rewards += tokens_per_nft.clone() * &user_nfts_number);
        }
    }

    fn distribute_to_tasc_stakers(&self, amount: &BigUint) {
        let number_nfts_staked = self.number_staked_second_collection().get();
        let number_olympian_staked = self.number_olympian_staked_second_collection().get();
        let number_normal_staked = &number_nfts_staked - &number_olympian_staked;

        let rewards_per_nft = amount / (number_normal_staked + (number_olympian_staked * 11 / 10));
        let rewards_per_olympian = &rewards_per_nft * &BigUint::from(11u32) / &BigUint::from(10u32);

        for user in self.users_staked_second_collection().iter() {
            let user_staked_nfts = self.user_staked_second_collection(&user);

            let user_staked_olympian = BigUint::from(self.user_number_staked_olympian_second_collection(&user).get());
            let user_staked_normal = &BigUint::from(user_staked_nfts.len()) - &user_staked_olympian;

            let user_rewards = &rewards_per_nft * &user_staked_normal + &rewards_per_olympian * &user_staked_olympian;
            self.user_rewards(&user).update(|rewards| *rewards += user_rewards);
        }
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        //get rewards to claim
        let caller = self.blockchain().get_caller();
        let rewards = self.user_rewards(&caller).get();

        //check if there are rewards to claim
        require!(rewards.clone() > 0, "No rewards to claim");

        //update storage
        self.user_rewards(&caller).clear();

        //send rewards
        self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::egld(), 0, &rewards);
    }


    #[view(userRewards)]
    #[storage_mapper("userRewards")]
    fn user_rewards(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;


    #[view(teamAddresses)]
    #[storage_mapper("teamAddresses")]
    fn team_addresses(&self) -> UnorderedSetMapper<TeamMember<Self::Api>>;


    #[only_owner]
    #[endpoint(setTeamAddresses)]
    fn set_team_addresses(&self, team_addresses: MultiValueEncoded<MultiValue2<ManagedAddress, u8>>) {
        self.team_addresses().clear();
        for team_address in team_addresses {
            let (address, percent) = team_address.into_tuple();
            let team_member = TeamMember {
                address: address,
                percent: percent,
            };
            self.team_addresses().insert(team_member);
        }
    }


    #[view(teamRoyaltiesSecondCollection)]
    #[storage_mapper("teamRoyaltiesSecondCollection")]
    fn team_royalties_second_collection(&self) -> SingleValueMapper<u8>;

    #[view(taccRoyaltiesSecondCollection)]
    #[storage_mapper("taccRoyaltiesSecondCollection")]
    fn tacc_royalties_second_collection(&self) -> SingleValueMapper<u8>;

    #[view(tascRoyaltiesSecondCollection)]
    #[storage_mapper("tascRoyaltiesSecondCollection")]
    fn tasc_royalties_second_collection(&self) -> SingleValueMapper<u8>;


    
    #[only_owner]
    #[endpoint(setTeamRoyaltiesSecondCollection)]
    fn set_team_royalties_second_collection(&self, r: u8) {
        self.team_royalties_second_collection().set(r);
    }

    #[only_owner]
    #[endpoint(setTaccRoyaltiesSecondCollection)]
    fn set_tacc_royalties_second_collection(&self, r: u8) {
        self.tacc_royalties_second_collection().set(r);
    }

    #[only_owner]
    #[endpoint(setTascRoyaltiesSecondCollection)]
    fn set_tasc_royalties_second_collection(&self, r: u8) {
        self.tasc_royalties_second_collection().set(r);
    }


    #[storage_mapper("stakingFirstCollectionIdentifier")]
    fn first_collection_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getNumberStakedFirstCollection)]
    #[storage_mapper("numberStakedFirstCollection")]
    fn number_staked_first_collection(&self) -> SingleValueMapper<u32>;

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

        self.stake_second_internal(caller, nonce);
    }

    #[payable("*")]
    #[endpoint(stakeMultipleSecond)]
    fn stake_multiple_second(&self,
        #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>) {
        
        let caller = self.blockchain().get_caller();
        let second_collection_token = self.second_collection_token().get();

        for payment in &payments {
            let token_payment = EgldOrEsdtTokenPayment::from(payment);

            require!(token_payment.token_identifier == second_collection_token, "Invalid NFT input, {} expected", second_collection_token);

            self.stake_second_internal(caller.clone(), token_payment.token_nonce);
        }
    }

    fn stake_second_internal(&self, user: ManagedAddress, nonce: u64) {
        self.users_staked_second_collection().insert(user.clone());
        self.user_staked_second_collection(&user).insert(nonce);
        self.number_staked_second_collection().update(|number| *number += 1);

        //check if is olympian
        if self.olympian_nonces().contains(&nonce) {
            self.number_olympian_staked_second_collection().update(|number| *number += 1);
            self.user_number_staked_olympian_second_collection(&user).update(|number| *number += 1);
        }
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
        self.number_staked_second_collection().update(|number| *number -= 1);

        //check if is olympian
        if self.olympian_nonces().contains(&nonce) {
            self.number_olympian_staked_second_collection().update(|number| *number -= 1);
            self.user_number_staked_olympian_second_collection(&caller).update(|number| *number -= 1);
        }
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
    
    #[endpoint(cancelUnstakeSecond)]
    fn cancel_unstake_second(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //get unstaking position
        let user_unstaked = self.user_unstaked_second_collection(&caller);
        let mut user_unstaked_iter = user_unstaked.iter();
        let unstaking_position = user_unstaked_iter.find(|position| position.nonce == nonce);
        
        //check if NFT was already unstaked
        require!(unstaking_position.is_some(), "NFT needs to be unstaked first");
        
        //update storage
        self.user_unstaked_second_collection(&caller).swap_remove(&unstaking_position.unwrap());
        self.number_unstaked_second_collection().update(|number| *number -= 1);

        //get NFT back to stake
        self.stake_second_internal(caller, nonce);
    }

    #[storage_mapper("stakingSecondCollectionIdentifier")]
    fn second_collection_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getNumberStakedSecondCollection)]
    #[storage_mapper("numberStakedSecondCollection")]
    fn number_staked_second_collection(&self) -> SingleValueMapper<u32>;

    #[view(getNumberOlympianStakedSecondCollection)]
    #[storage_mapper("numberOlympianStakedSecondCollection")]
    fn number_olympian_staked_second_collection(&self) -> SingleValueMapper<u32>;

    #[view(getUsersStakedSecondCollection)]
    #[storage_mapper("usersStakedSecondCollection")]
    fn users_staked_second_collection(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getUserStakedSecondCollection)]
    #[storage_mapper("userStakedSecondCollection")]
    fn user_staked_second_collection(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;
    
    #[view(getUserStakedOlympianSecondCollection)]
    #[storage_mapper("userStakedOlympianSecondCollection")]
    fn user_number_staked_olympian_second_collection(&self, user: &ManagedAddress) -> SingleValueMapper<u32>;
    
    #[view(getUserUnstakedSecondCollection)]
    #[storage_mapper("userUnstakedSecondCollection")]
    fn user_unstaked_second_collection(&self, user: &ManagedAddress) -> UnorderedSetMapper<UnstakingPosition>;

    #[view(getNumberUnstakedSecondCollection)]
    #[storage_mapper("numberUnstakedSecondCollection")]
    fn number_unstaked_second_collection(&self) -> SingleValueMapper<u32>;


    #[storage_mapper("olympianNonces")]
    fn olympian_nonces(&self) -> UnorderedSetMapper<u64>;

}
