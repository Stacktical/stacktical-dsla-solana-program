use anchor_lang::prelude::*;

use super::DslaDecimal;

/// collection fo all the parametric Governances one account for all SLAs
#[account]
pub struct Governance {
    /// amount of dsla to be deposited by the sla_deployer to deploy the sla for each period
    pub dsla_deposit_by_period: u128,
    /// percentage of dsla deposit by period to be given to the platform
    pub dsla_protocol_reward_rate: DslaDecimal,
    /// percentage of dsla deposit by period to be given to the validator
    pub dsla_validator_reward_rate: DslaDecimal,
    /// percentage of dsla deposit by period to be burned
    pub dsla_burned_by_verification_rate: DslaDecimal,
    /// percentage of withdrawal to be payed to the Deployer of the SLA
    pub sla_deployer_rewards_rate: DslaDecimal,
    /// percentage of withdrawal to be payed to the Deployer of the DSLA protocol
    pub protocol_rewards_rate: DslaDecimal,
    /// max leverage allowed in a DSLA
    pub max_leverage: DslaDecimal,
    /// boolean defining if burning of DSLA is on or off
    pub burn_dsla: bool,
}

impl Governance {
    pub const LEN: usize = 8  // discriminator
    + 16 // dsla_deposit_by_period 
    + 12 // dsla_protocol_reward_rate
    + 12 // dsla_validator_reward_rate
    + 12 // dsla_burned_by_verification_rate
    + 12 // sla_deployer_rewards_rate
    + 12 // protocol_rewards_rate
    + 12  // max_leverage
    + 1; // burn_dsla
}
