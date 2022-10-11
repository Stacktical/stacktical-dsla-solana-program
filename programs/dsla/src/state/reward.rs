use anchor_lang::prelude::*;

use super::DslaDecimal;

#[account]
pub struct Reward {
    last_claimed_period: u32,
    period_reward: u32,
}
