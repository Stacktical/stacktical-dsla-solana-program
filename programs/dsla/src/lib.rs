use anchor_lang::prelude::Result;
use anchor_lang::prelude::*;

/// storage for all the constants in the protocol
pub mod constants;
///  DSLA specific errors
pub mod errors;
///  DSLA specific events
pub mod events;
/// available instructions
pub mod instructions;
/// Accounts and structs used in the instructions
pub mod state;

use instructions::*;

use crate::state::sla::{DslaDecimal, PeriodLength, Slo};

declare_id!("HaTDBm8Ps7P6xBWFq5YbRUAnSwvCZNTceTuMB2VC3azv");

/// the main program
#[program]
pub mod dsla {
    use super::*;

    pub fn init_governance(
        ctx: Context<InitGovernance>,
        dsla_deposit_by_period: u64,
        dsla_protocol_reward: u64,
        dsla_validator_reward: u64,
        dsla_burned_by_verification: u64,
        sla_deployer_rewards_rate: DslaDecimal,
        protocol_rewards_rate: DslaDecimal,
        max_leverage: DslaDecimal,
    ) -> Result<()> {
        instructions::init_governance::handler(
            ctx,
            dsla_deposit_by_period,
            dsla_protocol_reward,
            dsla_validator_reward,
            dsla_burned_by_verification,
            sla_deployer_rewards_rate,
            protocol_rewards_rate,
            max_leverage,
        )
    }

    pub fn modify_governance(
        ctx: Context<ModifyGovernance>,
        dsla_deposit_by_period: u64,
        dsla_protocol_reward: u64,
        dsla_validator_reward: u64,
        dsla_burned_by_verification: u64,
        sla_deployer_rewards_rate: DslaDecimal,
        protocol_rewards_rate: DslaDecimal,
        max_leverage: DslaDecimal,
    ) -> Result<()> {
        instructions::modify_governance::handler(
            ctx,
            dsla_deposit_by_period,
            dsla_protocol_reward,
            dsla_validator_reward,
            dsla_burned_by_verification,
            sla_deployer_rewards_rate,
            protocol_rewards_rate,
            max_leverage,
        )
    }

    pub fn init_sla_registry(ctx: Context<InitSlaRegistry>) -> Result<()> {
        instructions::init_sla_registry::handler(ctx)
    }

    pub fn stake_user(ctx: Context<StakeUser>, token_amount: u64) -> Result<()> {
        instructions::stake_user::handler(ctx, token_amount)
    }

    pub fn stake_provider(ctx: Context<StakeProvider>, token_amount: u64) -> Result<()> {
        instructions::stake_provider::handler(ctx, token_amount)
    }

    pub fn validate_period(ctx: Context<ValidatePeriod>, period: u64) -> Result<()> {
        instructions::validate_period::handler(ctx, period as usize)
    }

    pub fn withdraw_user(ctx: Context<WithdrawUser>, token_amount: u64) -> Result<()> {
        instructions::withdraw_user::handler(ctx, token_amount)
    }

    pub fn withdraw_provider(ctx: Context<WithdrawProvider>, token_amount: u64) -> Result<()> {
        instructions::withdraw_provider::handler(ctx, token_amount)
    }

    pub fn init_lockup_accounts(ctx: Context<InitLockupAccounts>) -> Result<()> {
        instructions::init_lockup_accounts::handler(ctx)
    }

    pub fn deploy_sla(
        ctx: Context<DeploySla>,
        slo: Slo,
        leverage: DslaDecimal,
        start: u128,
        n_periods: u32,
        period_length: PeriodLength,
    ) -> Result<()> {
        instructions::deploy_sla::handler(ctx, slo, leverage, start, n_periods, period_length)
    }
}
