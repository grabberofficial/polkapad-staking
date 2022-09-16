use codec::Encode;
use gstd::{String};
use gtest::{Program, System};

use staking_io::*;
use ft_io::*;

const TOKEN_ADDRESS: u64 = 1;
const STAKING_ADDRESS: u64 = 2;

const DEPLOYER: u64 = 10;
const ALICE: u64 = 11;

#[test]
fn stake_50_plpd_should_staked() {
    let system = System::new();
    init_staking_token(&system);
    init_staking(&system);
    system.init_logger();

    let to_stake: u128 = 50;
    let expected_staked: u128 = 50;

    let token = system.get_program(TOKEN_ADDRESS);
    let staking = system.get_program(STAKING_ADDRESS);

    let result = staking.send(ALICE, StakingAction::Stake(to_stake));
    assert!(result.contains(&(ALICE, StakingEvent::Staked(expected_staked).encode())));

    let result = token.send(ALICE, FTAction::BalanceOf(ALICE.into()));
    assert!(result.contains(&(ALICE, FTEvent::Balance(to_stake).encode())));

    let result = token.send(ALICE, FTAction::BalanceOf(STAKING_ADDRESS.into()));
    assert!(result.contains(&(ALICE, FTEvent::Balance(expected_staked).encode())));
}

#[test]
fn stake_150_plpd_should_failed() {
    let system = System::new();
    init_staking_token(&system);
    init_staking(&system);
    system.init_logger();

    let to_stake: u128 = 150;

    let staking = system.get_program(STAKING_ADDRESS);

    let result = staking.send(ALICE, StakingAction::Stake(to_stake));
    assert!(result.main_failed())
}

#[test]
fn stake_0_plpd_should_failed() {
    let system = System::new();
    init_staking_token(&system);
    init_staking(&system);
    system.init_logger();

    let to_stake: u128 = 0;

    let staking = system.get_program(STAKING_ADDRESS);

    let result = staking.send(ALICE, StakingAction::Stake(to_stake));
    assert!(result.main_failed())
}

#[test]
fn stake_50_then_withraw_25_should_withdrawed_25() {
    let system = System::new();
    init_staking_token(&system);
    init_staking(&system);
    system.init_logger();

    let to_stake: u128 = 50;
    let expected_staked: u128 = 50;
    
    let to_withdraw: u128 = 25;
    let expected_withdraw: u128 = 25;
    let expected_balance: u128 = 75;

    let token = system.get_program(TOKEN_ADDRESS);
    let staking = system.get_program(STAKING_ADDRESS);

    let result = staking.send(ALICE, StakingAction::Stake(to_stake));
    assert!(result.contains(&(ALICE, StakingEvent::Staked(expected_staked).encode())));

    let result = staking.send(ALICE, StakingAction::Withdraw(to_withdraw));
    assert!(result.contains(&(ALICE, StakingEvent::Withdrawed(expected_withdraw).encode())));

    let result = token.send(ALICE, FTAction::BalanceOf(ALICE.into()));
    assert!(result.contains(&(ALICE, FTEvent::Balance(expected_balance).encode())));

    let result = staking.send(ALICE, StakingAction::StakeOf(ALICE.into()));
    assert!(result.contains(&(ALICE, StakingEvent::Staked(expected_staked - to_withdraw).encode())));

    let result = token.send(ALICE, FTAction::BalanceOf(STAKING_ADDRESS.into()));
    assert!(result.contains(&(ALICE, FTEvent::Balance(expected_withdraw).encode())));
}

#[test]
fn stake_50_then_withdraw_75_should_failed() {
    let system = System::new();
    init_staking_token(&system);
    init_staking(&system);
    system.init_logger();

    let to_stake: u128 = 50;
    let expected_staked: u128 = 50;
    
    let to_withdaw: u128 = 75;

    let staking = system.get_program(STAKING_ADDRESS);

    let result = staking.send(ALICE, StakingAction::Stake(to_stake));
    assert!(result.contains(&(ALICE, StakingEvent::Staked(expected_staked).encode())));

    let result = staking.send(ALICE, StakingAction::Withdraw(to_withdaw));
    assert!(result.main_failed());
}

fn init_staking(system: &System) {
    let staking = Program::current(system);

    let result = staking.send(
        DEPLOYER,
        StakingInitialConfiguration {
            token_address: TOKEN_ADDRESS.into()
        },
    );

    assert!(result.log().is_empty());
}

fn init_staking_token(system: &System) {
    let token = Program::from_file(system, "../fungible-token/target/wasm32-unknown-unknown/release/fungible_token.wasm");

    let result = token.send(
        DEPLOYER,
        FTInitialConfiguration {
            name: String::from("TestToken"),
            symbol: String::from("TPLPD"),
            decimals: 18
        },
    );

    assert!(!result.main_failed());
    
    token.send(
        DEPLOYER,
        FTAction::TransferFrom {
            from: DEPLOYER.into(),
            to: ALICE.into(),
            amount: 100
        },
    );
    
    let result = token.send(ALICE, FTAction::BalanceOf(ALICE.into()));
    assert!(result.contains(&(ALICE, FTEvent::Balance(100).encode())));
}