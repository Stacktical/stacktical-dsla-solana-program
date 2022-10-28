use anchor_lang::prelude::*;

use super::Side;

#[account]
pub struct Reward {
    pub last_claimed_period: LastClaimedPeriod,
    pub future_periods_reward: u64,
    pub current_period_reward: u64,
    pub side: Side,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum LastClaimedPeriod {
    NeverClaimed,
    Claimed { last_claimed_period: usize },
}

impl Reward {
    pub const LEN: usize = 8 + 1 + 4 + 16 + 16 + 16;
}
