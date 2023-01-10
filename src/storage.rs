elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::TeamMember;
use crate::UnstakingPosition;

#[elrond_wasm::module]
pub trait Storage {
    /*
     ***** GENERIC *****
     */
    #[view(userRewards)]
    #[storage_mapper("userRewards")]
    fn user_rewards(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(teamAddresses)]
    #[storage_mapper("teamWallets")]
    fn team_addresses(&self) -> UnorderedSetMapper<TeamMember<Self::Api>>;

    #[view(teamRoyaltiesSecondCollection)]
    #[storage_mapper("teamRoyaltiesSecondCollection")]
    fn team_royalties_second_collection(&self) -> SingleValueMapper<u8>;

    /*
     ***** TACC *****
     */
    #[view(taccRoyaltiesSecondCollection)]
    #[storage_mapper("taccRoyaltiesSecondCollection")]
    fn tacc_royalties_second_collection(&self) -> SingleValueMapper<u8>;

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
    fn user_unstaked_first_collection(
        &self,
        user: &ManagedAddress,
    ) -> UnorderedSetMapper<UnstakingPosition>;

    #[view(getNumberUnstakedFirstCollection)]
    #[storage_mapper("numberUnstakedFirstCollection")]
    fn number_unstaked_first_collection(&self) -> SingleValueMapper<u32>;

    /*
     ***** TASC *****
     */
    #[view(tascRoyaltiesSecondCollection)]
    #[storage_mapper("tascRoyaltiesSecondCollection")]
    fn tasc_royalties_second_collection(&self) -> SingleValueMapper<u8>;

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
    fn user_number_staked_olympian_second_collection(
        &self,
        user: &ManagedAddress,
    ) -> SingleValueMapper<u32>;

    #[view(getUserUnstakedSecondCollection)]
    #[storage_mapper("userUnstakedSecondCollection")]
    fn user_unstaked_second_collection(
        &self,
        user: &ManagedAddress,
    ) -> UnorderedSetMapper<UnstakingPosition>;

    #[view(getNumberUnstakedSecondCollection)]
    #[storage_mapper("numberUnstakedSecondCollection")]
    fn number_unstaked_second_collection(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("olympianNonces")]
    fn olympian_nonces(&self) -> UnorderedSetMapper<u64>;
}
