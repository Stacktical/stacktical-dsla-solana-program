use anchor_lang::prelude::Result;
use anchor_lang::prelude::*;

/// storage for all the constants in the protocol
pub mod constants;
/// all the DSLA specific errors
pub mod errors;
/// all the DSLA specific events
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

use crate::state::governance::Governance;
use crate::state::sla::{DslaDecimal, PeriodLength, Slo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
/// the main program
#[program]
pub mod dsla {
    use super::*;

    pub fn init_sla_registry(
        ctx: Context<InitSlaRegistry>,
        governance_parameters: Governance,
    ) -> Result<()> {
        instructions::init_sla_registry::handler(ctx, governance_parameters)
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

// @remind deal with all the unwrap() troughout the code
