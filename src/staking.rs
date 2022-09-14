use gstd::{exec, msg, prelude::*, ActorId};
use ft_io::FTAction;
use staking_io::{Staker, StakingInitialConfiguration, StakingEvent};

#[derive(Debug, Default)]
pub struct Staking {
    pub owner: ActorId,
    pub token_address: ActorId,
    pub stakers: BTreeMap<ActorId, Staker>,
    pub total_staked: u128
}

impl Staking {
    pub async fn stake(&mut self, amount: u128) {
        if amount == 0 {
            panic!("Polkapad Staking: amount must be greater than 0");
        }

        transfer_tokens(
            &self.token_address, 
            &msg::source(), 
            &exec::program_id(), 
            amount)
            .await;

        self.stakers
            .entry(msg::source())
            .and_modify(|stake| {
                stake.balance = stake.balance.saturating_add(amount);
            })
            .or_insert(Staker {
                balance: amount
            });

        self.total_staked = self.total_staked.saturating_add(amount);

        msg::send_for_reply(
            msg::source(), 
            StakingEvent::Staked(amount),
            0)
            .expect("Polkapad Staking: 'stake' error")
            .await
            .expect("Operation suspended");
}

    pub async fn withdraw(&mut self, amount: u128) {
        if amount == 0 {
            panic!("Polkapad Staking: amount must be great than 0");
        }

        let staker = self
            .stakers
            .get_mut(&msg::source())
            .unwrap_or_else(|| panic!("Polkapad Staking: staker {:?} not found", msg::source()));

        if staker.balance < amount {
            panic!("Polkapad Staking: staker balance is '{:?}'. This is less than the withdraw amount", staker.balance);
        }

        transfer_tokens(
            &self.token_address, 
            &exec::program_id(), 
            &msg::source(), 
            amount)
            .await;

        staker.balance = staker.balance.saturating_sub(amount);
        self.total_staked = self.total_staked.saturating_sub(amount);

        msg::send_for_reply(
        msg::source(), 
        StakingEvent::Withdrawed(amount), 
        0)
            .expect("Polkapad Staking: 'widthraw' error")
            .await
            .expect("Operation suspended");
    }

    pub fn stake_of(&self, staker_id: ActorId) {
        let staker = self.stakers
                .get(&staker_id)
                .unwrap_or(&Staker { balance: 0 });

        msg::reply(StakingEvent::StakeOf(*staker), 0);
    }

    pub fn update_configuration(&mut self, configuration: StakingInitialConfiguration) {
        if msg::source() != self.owner {
            panic!("Polkapad Staking: only 'owner' address");
        }

        self.token_address = configuration.token_address;
    }
}

async fn transfer_tokens(
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount: u128,
) {
    let reply = msg::send_for_reply(
        *token_address,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .expect("Polkapad Staking: error in sending message")
    .await;

    match reply {
        Ok(_) => {},
        Err(msg) => panic!("{:?}", msg)
    }
}