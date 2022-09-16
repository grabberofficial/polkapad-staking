#![no_std]

#[cfg(test)]
mod tests;
mod staking;

use gstd::{msg, prelude::*};

use staking::Staking;
use staking_io::*;

static mut STAKING: Option<Staking> = None;

gstd::metadata! {
    title: "PolkapadStaking",
    init:
        input: StakingInitialConfiguration,
    handle:
        input: StakingAction,
        output: StakingEvent,
    state:
        input: StakingState,
        output: StakingReply,
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: StakingInitialConfiguration = msg::load()
        .expect("Polkapad Staking: Unable to decode configuration");

    let staking = Staking {
        owner: msg::source(),
        token_address: config.token_address,
        ..Staking::default()
    };

    STAKING = Some(staking);
}

#[gstd::async_main]
async unsafe fn main() {
    let staking = unsafe { STAKING.get_or_insert(Staking::default()) };

    let action: StakingAction = msg::load().expect("Polkapad Staking: could not load Action");
    match action {
        StakingAction::Stake(amount) => {
            staking.stake(amount).await;
        }
        StakingAction::Withdraw(amount) => {
            staking.withdraw(amount).await;
        },
        StakingAction::StakeOf(staker) => {
            staking.stake_of(staker);
        },
        StakingAction::UpdateConfiguration(configuration) => {
            staking.update_configuration(configuration);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: StakingState = msg::load()
        .expect("Polkapad Staking: Failed to decode input argument");

    let staking: &mut Staking = STAKING.get_or_insert(Staking::default());
    
    let encoded = match query {
        StakingState::Owner => StakingReply::Owner(staking.owner),
        StakingState::StakeOf(staker_id) => {
            let staker = staking.stakers
                .get(&staker_id)
                .unwrap_or(&Staker { balance: 0 });

            StakingReply::Staked(staker.balance)
        },
        StakingState::TotalStaked => StakingReply::TotalStaked(staking.total_staked),
        StakingState::TokenAddress => StakingReply::TokenAddress(staking.token_address)
    }
    .encode();

    gstd::util::to_leak_ptr(encoded)
}