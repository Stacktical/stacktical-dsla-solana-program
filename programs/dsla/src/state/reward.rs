use anchor_lang::prelude::*;

#[account]
pub struct Reward {
    last_claimed_period: u32,
    future_periods_reward: u128,
    previous_periods_reward: u128,
    current_period_reward: u128,
}
