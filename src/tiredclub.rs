#![no_std]
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const UNBOND_DAYS: u64 = 6;
const HOUR_IN_SECONDS: u64 = 60 * 60;
const DAY_IN_SECONDS: u64 = 24 * HOUR_IN_SECONDS;

pub mod dao;
pub mod storage;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct UnstakingPosition {
    pub nonce: u64,
    pub deadline: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct TeamMember<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub percent: u16,
}

#[elrond_wasm::contract]
pub trait TiredClub: elrond_wasm_modules::dns::DnsModule + dao::Dao + storage::Storage {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(migrateWallet)]
    fn migrate_wallet(&self, wallet_from: ManagedAddress, wallet_to: ManagedAddress) {
        self.user_rewards(&wallet_to).update(|rewards| {
            *rewards += &self.user_rewards(&wallet_from).get();
        });
        self.user_rewards(&wallet_from).clear();

        if self.users_staked_first_collection().contains(&wallet_from) {
            self.users_staked_first_collection()
                .insert(wallet_to.clone());
            self.users_staked_first_collection()
                .swap_remove(&wallet_from);

            let user_staked_first_collection = self.user_staked_first_collection(&wallet_from);
            for nonce in user_staked_first_collection.iter() {
                self.user_staked_first_collection(&wallet_to).insert(nonce);
            }
            self.user_staked_first_collection(&wallet_from).clear();

            let user_unstaked_first_collection = self.user_unstaked_first_collection(&wallet_from);
            for position in user_unstaked_first_collection.iter() {
                self.user_unstaked_first_collection(&wallet_to)
                    .insert(position);
            }
            self.user_unstaked_first_collection(&wallet_from).clear();
        }

        if self.users_staked_second_collection().contains(&wallet_from) {
            self.users_staked_second_collection()
                .insert(wallet_to.clone());
            self.users_staked_second_collection()
                .swap_remove(&wallet_from);

            let user_staked_second_collection = self.user_staked_second_collection(&wallet_from);
            for nonce in user_staked_second_collection.iter() {
                self.user_staked_second_collection(&wallet_to).insert(nonce);
            }
            self.user_staked_second_collection(&wallet_from).clear();

            let user_unstaked_second_collection =
                self.user_unstaked_second_collection(&wallet_from);
            for position in user_unstaked_second_collection.iter() {
                self.user_unstaked_second_collection(&wallet_to)
                    .insert(position);
            }
            self.user_unstaked_second_collection(&wallet_from).clear();

            self.user_number_staked_olympian_second_collection(&wallet_to)
                .update(|number| {
                    *number += &self
                        .user_number_staked_olympian_second_collection(&wallet_from)
                        .get();
                });
            self.user_number_staked_olympian_second_collection(&wallet_from)
                .clear();

            self.user_number_staked_legendary_second_collection(&wallet_to)
                .update(|number| {
                    *number += &self
                        .user_number_staked_legendary_second_collection(&wallet_from)
                        .get();
                });
            self.user_number_staked_legendary_second_collection(&wallet_from)
                .clear();
        }
    }

    /*
     ***** FIRST COLLECTION - TACC *****
     */
    #[payable("*")]
    #[endpoint(stakeFirst)]
    fn stake_first(
        &self,
        #[payment_token] payment_token: EgldOrEsdtTokenIdentifier,
        #[payment_nonce] nonce: u64,
    ) {
        let caller = self.blockchain().get_caller();
        let first_collection_token = self.first_collection_token().get();

        require!(
            payment_token == first_collection_token,
            "Invalid NFT input, {} expected",
            first_collection_token
        );

        self.stake_first_internal(caller, nonce)
    }

    #[payable("*")]
    #[endpoint(stakeMultipleFirst)]
    fn stake_multiple_first(
        &self,
        #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
    ) {
        let caller = self.blockchain().get_caller();
        let first_collection_token = self.first_collection_token().get();

        for payment in &payments {
            let token_payment = EgldOrEsdtTokenPayment::from(payment);

            require!(
                token_payment.token_identifier == first_collection_token,
                "Invalid NFT input, {} expected",
                first_collection_token
            );

            self.stake_first_internal(caller.clone(), token_payment.token_nonce);
        }
    }

    fn stake_first_internal(&self, user: ManagedAddress, nonce: u64) {
        self.users_staked_first_collection().insert(user.clone());
        self.user_staked_first_collection(&user).insert(nonce);
        self.number_staked_first_collection()
            .update(|number| *number += 1);
    }

    #[endpoint(unstakeFirst)]
    fn unstake_first(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //check if nonce is staked and remove it
        let is_nonce_staked = self
            .user_staked_first_collection(&caller)
            .swap_remove(&nonce);
        require!(is_nonce_staked, "NFT was not staked");

        let unstaking_position = UnstakingPosition {
            nonce,
            deadline: self.blockchain().get_block_timestamp() + UNBOND_DAYS * DAY_IN_SECONDS,
        };

        //update storage
        self.user_unstaked_first_collection(&caller)
            .insert(unstaking_position);
        self.number_unstaked_first_collection()
            .update(|number| *number += 1);
        self.number_staked_first_collection()
            .update(|number| *number -= 1);
    }

    #[endpoint(withdrawFirst)]
    fn withdraw_first(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //get unstaking position
        let user_unstaked = self.user_unstaked_first_collection(&caller);
        let mut user_unstaked_iter = user_unstaked.iter();
        let unstaking_position = user_unstaked_iter.find(|position| position.nonce == nonce);

        //check if NFT was already unstaked
        require!(
            unstaking_position.is_some(),
            "NFT needs to be unstaked first"
        );
        //check if deadline is passed
        require!(
            unstaking_position.as_ref().unwrap().deadline < self.blockchain().get_block_timestamp(),
            "NFT is still locked"
        );

        //update storage
        self.user_unstaked_first_collection(&caller)
            .swap_remove(&unstaking_position.unwrap());
        self.number_unstaked_first_collection()
            .update(|number| *number -= 1);

        //send NFT back to user
        let first_collection_token = self.first_collection_token().get();
        self.send().direct(
            &caller,
            &first_collection_token,
            nonce,
            &BigUint::from(1u32),
        );

        //check if user has no more NFTs staked
        if self.user_staked_first_collection(&caller).len() == 0 {
            self.users_staked_first_collection().swap_remove(&caller);
        }
    }

    #[endpoint(cancelUnstakeFirst)]
    fn cancel_unstake_first(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //get unstaking position
        let user_unstaked = self.user_unstaked_first_collection(&caller);
        let mut user_unstaked_iter = user_unstaked.iter();
        let unstaking_position = user_unstaked_iter.find(|position| position.nonce == nonce);

        //check if NFT was already unstaked
        require!(
            unstaking_position.is_some(),
            "NFT needs to be unstaked first"
        );

        //update storage
        self.user_unstaked_first_collection(&caller)
            .swap_remove(&unstaking_position.unwrap());
        self.number_unstaked_first_collection()
            .update(|number| *number -= 1);

        //get NFT back to stake
        self.stake_first_internal(caller, nonce);
    }

    #[payable("EGLD")]
    #[endpoint(distributeRoyaltiesFirstCollection)]
    #[only_owner]
    fn distribute_royalties_first_collection(&self, #[payment_amount] payment_amount: BigUint) {
        self.distribute_to_team(&payment_amount);
    }

    #[payable("EGLD")]
    #[endpoint(distributeRoyaltiesSecondCollection)]
    #[only_owner]
    fn distribute_royalties_second_collection(&self, #[payment_amount] payment_amount: BigUint) {
        let team_royalties_second_collection = self.team_royalties_second_collection().get();
        let team_amount = &payment_amount * &BigUint::from(team_royalties_second_collection)
            / &BigUint::from(100u8);

        /*
                let tasc_royalties_second_collection = self.tasc_royalties_second_collection().get();
                let tasc_amount = &payment_amount * &BigUint::from(tasc_royalties_second_collection)
                    / &BigUint::from(100u8);
        */

        let tacc_amount = payment_amount - &team_amount /* - &tasc_amount */;

        self.distribute_to_team(&team_amount);
        //self.distribute_to_tasc_stakers(&tasc_amount);
        self.distribute_to_tacc_stakers(&tacc_amount);
    }

    #[payable("EGLD")]
    #[endpoint(distributeToTacc)]
    #[only_owner]
    fn distribute_to_tacc(&self, #[payment_amount] payment_amount: BigUint) {
        self.distribute_to_tacc_stakers(&payment_amount);
    }

    #[payable("EGLD")]
    #[endpoint(distributeToTasc)]
    #[only_owner]
    fn distribute_to_tasc(&self, #[payment_amount] payment_amount: BigUint) {
        self.distribute_to_tasc_stakers(&payment_amount);
    }

    fn distribute_to_team(&self, amount: &BigUint) {
        for team_member in self.team_addresses().iter() {
            let member_amount =
                amount * &BigUint::from(team_member.percent) / &BigUint::from(1000u16);
            self.user_rewards(&team_member.address)
                .update(|rewards| *rewards += member_amount);
        }
    }

    fn distribute_to_tacc_stakers(&self, amount: &BigUint) {
        let number_nfts_staked = self.number_staked_first_collection().get();
        let tokens_per_nft = amount / number_nfts_staked;

        for user_staked in self.users_staked_first_collection().iter() {
            let user_nfts_number =
                BigUint::from(self.user_staked_first_collection(&user_staked).len());
            self.user_rewards(&user_staked)
                .update(|rewards| *rewards += tokens_per_nft.clone() * &user_nfts_number);
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

            let user_staked_olympian = BigUint::from(
                self.user_number_staked_olympian_second_collection(&user)
                    .get(),
            );
            let user_staked_normal = &BigUint::from(user_staked_nfts.len()) - &user_staked_olympian;

            let user_rewards = &rewards_per_nft * &user_staked_normal
                + &rewards_per_olympian * &user_staked_olympian;
            self.user_rewards(&user)
                .update(|rewards| *rewards += user_rewards);
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
        self.send()
            .direct(&caller, &EgldOrEsdtTokenIdentifier::egld(), 0, &rewards);
    }

    #[endpoint(compoundRewards)]
    fn compound_rewards(&self) {
        //get rewards to claim
        let caller = self.blockchain().get_caller();
        let rewards = self.user_rewards(&caller).get();

        //check if there are rewards to claim
        require!(rewards.clone() > 0, "No rewards to claim");

        //update storage
        self.user_rewards(&caller).clear();

        //compound rewards
        self.user_compounded_rewards(&caller)
            .update(|r| *r += rewards.clone());

        //update total compound rewards
        self.total_compound_rewards()
            .update(|r| *r += rewards.clone());

        //send rewards if total compound rewards > 0.5 EGLD
        let total_rewards = self.total_compound_rewards().get();
        if total_rewards.clone() > self.minimum_compound_to_send().get() {
            let owner = self.blockchain().get_owner_address();
            self.send().direct(
                &owner,
                &EgldOrEsdtTokenIdentifier::egld(),
                0,
                &total_rewards,
            );
            self.total_compound_rewards().clear();
        }
    }

    #[only_owner]
    #[endpoint(setMinimumCompoundToSend)]
    fn set_minimum_compound_to_send(&self, minimum_compound_to_send: BigUint) {
        self.minimum_compound_to_send()
            .set(&minimum_compound_to_send);
    }

    #[only_owner]
    #[endpoint(setTeamAddresses)]
    fn set_team_addresses(
        &self,
        team_addresses: MultiValueEncoded<MultiValue2<ManagedAddress, u16>>,
    ) {
        if self.team_addresses().len() > 0 {
            self.team_addresses().clear();
        }
        for team_address in team_addresses {
            let (address, percent) = team_address.into_tuple();
            let team_member = TeamMember {
                address: address,
                percent: percent,
            };
            self.team_addresses().insert(team_member);
        }
    }

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

    /*
     ***** SECOND COLLECTION - TASC *****
     */

    #[payable("*")]
    #[endpoint(stakeSecond)]
    fn stake_second(
        &self,
        #[payment_token] payment_token: EgldOrEsdtTokenIdentifier,
        #[payment_nonce] nonce: u64,
    ) {
        let caller = self.blockchain().get_caller();
        let second_collection_token = self.second_collection_token().get();

        require!(
            payment_token == second_collection_token,
            "Invalid NFT input, {} expected",
            second_collection_token
        );

        self.stake_second_internal(caller, nonce);
    }

    #[payable("*")]
    #[endpoint(stakeMultipleSecond)]
    fn stake_multiple_second(
        &self,
        #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
    ) {
        let caller = self.blockchain().get_caller();
        let second_collection_token = self.second_collection_token().get();

        for payment in &payments {
            let token_payment = EgldOrEsdtTokenPayment::from(payment);

            require!(
                token_payment.token_identifier == second_collection_token,
                "Invalid NFT input, {} expected",
                second_collection_token
            );

            self.stake_second_internal(caller.clone(), token_payment.token_nonce);
        }
    }

    fn stake_second_internal(&self, user: ManagedAddress, nonce: u64) {
        self.users_staked_second_collection().insert(user.clone());
        self.user_staked_second_collection(&user).insert(nonce);
        self.number_staked_second_collection()
            .update(|number| *number += 1);

        //check if is olympian
        if self.olympian_nonces().contains(&nonce) {
            self.number_olympian_staked_second_collection()
                .update(|number| *number += 1);
            self.user_number_staked_olympian_second_collection(&user)
                .update(|number| *number += 1);
        }

        //check if is legendary
        if self.legendary_nonces().contains(&nonce) {
            self.number_legendary_staked_second_collection()
                .update(|number| *number += 1);
            self.user_number_staked_legendary_second_collection(&user)
                .update(|number| *number += 1);
        }
    }

    #[endpoint(unstakeSecond)]
    fn unstake_second(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //check if nonce is staked and remove it
        let is_nonce_staked = self
            .user_staked_second_collection(&caller)
            .swap_remove(&nonce);
        require!(is_nonce_staked, "NFT was not staked");

        let unstaking_position = UnstakingPosition {
            nonce,
            deadline: self.blockchain().get_block_timestamp() + UNBOND_DAYS * DAY_IN_SECONDS,
        };

        //update storage
        self.user_unstaked_second_collection(&caller)
            .insert(unstaking_position);
        self.number_unstaked_second_collection()
            .update(|number| *number += 1);
        self.number_staked_second_collection()
            .update(|number| *number -= 1);

        //check if is olympian
        if self.olympian_nonces().contains(&nonce) {
            self.number_olympian_staked_second_collection()
                .update(|number| *number -= 1);
            self.user_number_staked_olympian_second_collection(&caller)
                .update(|number| *number -= 1);
        }

        //check if is legendary
        if self.legendary_nonces().contains(&nonce) {
            self.number_legendary_staked_second_collection()
                .update(|number| *number -= 1);
            self.user_number_staked_legendary_second_collection(&caller)
                .update(|number| *number -= 1);
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
        require!(
            unstaking_position.is_some(),
            "NFT needs to be unstaked first"
        );
        //check if deadline is passed
        require!(
            unstaking_position.as_ref().unwrap().deadline < self.blockchain().get_block_timestamp(),
            "NFT is still locked"
        );

        //update storage
        self.user_unstaked_second_collection(&caller)
            .swap_remove(&unstaking_position.unwrap());
        self.number_unstaked_second_collection()
            .update(|number| *number -= 1);

        //send NFT back to user
        let second_collection_token = self.second_collection_token().get();
        self.send().direct(
            &caller,
            &second_collection_token,
            nonce,
            &BigUint::from(1u32),
        );

        //check if user has no more NFTs staked
        if self.user_staked_second_collection(&caller).len() == 0 {
            self.users_staked_second_collection().swap_remove(&caller);
        }
    }

    #[endpoint(cancelUnstakeSecond)]
    fn cancel_unstake_second(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        //get unstaking position
        let user_unstaked = self.user_unstaked_second_collection(&caller);
        let mut user_unstaked_iter = user_unstaked.iter();
        let unstaking_position = user_unstaked_iter.find(|position| position.nonce == nonce);

        //check if NFT was already unstaked
        require!(
            unstaking_position.is_some(),
            "NFT needs to be unstaked first"
        );

        //update storage
        self.user_unstaked_second_collection(&caller)
            .swap_remove(&unstaking_position.unwrap());
        self.number_unstaked_second_collection()
            .update(|number| *number -= 1);

        //get NFT back to stake
        self.stake_second_internal(caller, nonce);
    }
}
