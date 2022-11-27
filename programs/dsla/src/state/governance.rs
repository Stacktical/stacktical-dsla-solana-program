use anchor_lang::prelude::*;

use super::DslaDecimal;

/// collection fo all the parametric Governances one account for all SLAs
#[account]
pub struct Governance {
    pub dsla_burn_rate: u128,
    pub dsla_deposit_by_period: u128,
    pub dsla_platform_reward: u128,
    pub dsla_messenger_reward: u128,
    pub dsla_user_reward: u128,
    pub dsla_burned_by_verification: u128,
    pub max_token_length: u128,
    /// max leverage allowed in a DSLA
    pub max_leverage: DslaDecimal,
    /// boolean defining if burning of DSLA is on or off
    pub burn_dsla: bool,
}

impl Governance {
    pub const LEN: usize = 8  // discriminator
    + 16 // dsla_burn_rate
    + 16 // dsla_deposit_by_period 
    + 16 // dsla_platform_reward
    + 16 // dsla_messenger_reward
    + 16 // dsla_user_reward
    + 16 // dsla_burned_by_verification
    + 16 // max_token_length
    + 12  // max_leverage
    + 1; // burn_dsla
}
