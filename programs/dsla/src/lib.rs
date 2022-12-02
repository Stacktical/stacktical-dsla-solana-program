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

use crate::state::governance::Governance;
use crate::state::sla::{DslaDecimal, PeriodLength, Slo};

declare_id!("DDqoT9zs2YCd4SkL2MYuB8KBbBLowstkT38pdAoM5yXA");
/// the main program
#[program]
pub mod dsla {
    use super::*;

    pub fn init_governance(
        ctx: Context<InitGovernance>,
        governance_parameters: Governance,
    ) -> Result<()> {
        instructions::init_governance::handler(ctx, governance_parameters)
    }

    pub fn modify_governance(
        ctx: Context<ModifyGovernance>,
        governance_parameters: Governance,
    ) -> Result<()> {
        instructions::modify_governance::handler(ctx, governance_parameters)
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
        messenger_address: Pubkey,
        leverage: DslaDecimal,
        start: u128,
        n_periods: u32,
        period_length: PeriodLength,
    ) -> Result<()> {
        instructions::deploy_sla::handler(
            ctx,
            slo,
            messenger_address,
            leverage,
            start,
            n_periods,
            period_length,
        )
    }
}
