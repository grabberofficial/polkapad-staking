#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct StakingInitialConfiguration {
    pub token_address: ActorId
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Copy, Clone, PartialEq)]
pub struct Staker {
    pub balance: u128
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StakingAction {
    Stake(u128),
    Withdraw(u128),
    StakeOf(ActorId),
    UpdateConfiguration(StakingInitialConfiguration),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StakingEvent {
    Staked(u128),
    Withdrawed(u128),
    ConfigurationUpdate
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StakingState {
    Owner,
    StakeOf(ActorId),
    TotalStaked,
    TokenAddress
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StakingReply {
    Owner(ActorId),
    Staked(u128),
    TotalStaked(u128),
    TokenAddress(ActorId)
}
