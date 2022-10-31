use anchor_lang::prelude::*;

use super::Side;

/// each individual account has it's own reward stored as derived from the amount staked
#[account]
pub struct Reward {
    /// the id of the last claimed period
    pub last_claimed_period: LastClaimedPeriod,
    /// the rewards for all the claimable periods except the first one
    pub future_periods_reward: u64,
    /// the reward for the first claimable period
    pub current_period_reward: u64,
    /// the side of where the reward is on
    pub side: Side,
}

/// period can be neverclaimed or Claimed
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum LastClaimedPeriod {
    /// no claims have been done before, this is not equal to `last_claimed_period = 0`
    NeverClaimed,
    /// there have been claims before with period id `last_claimed_period`
    Claimed { last_claimed_period: usize },
}

impl Reward {
    pub const LEN: usize = 8 + 1 + 4 + 16 + 16 + 16;
}
