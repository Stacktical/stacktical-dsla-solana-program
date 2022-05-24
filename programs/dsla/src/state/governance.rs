use anchor_lang::prelude::*;


#[account]
pub struct Governance {
    pub dsla_burn_rate: u128,
    pub dsla_deposit_by_period: u128,
    pub dsla_platform_reward: u128,
    pub dsla_messenger_reward: u128,
    pub dsla_user_reward: u128,
    pub dsla_burned_by_verification: u128,
    pub max_token_length: u128,
    pub max_leverage: u64,
    pub burn_dsla : bool,
}