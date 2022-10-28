use anchor_lang::prelude::Result;
use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

use crate::state::governance::Governance;
use crate::state::sla::{DslaDecimal, PeriodLength, Slo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod dsla {
    use super::*;

    pub fn init_sla_registry(
        ctx: Context<InitSlaRegistry>,
        governance_parameters: Governance,
    ) -> Result<()> {
        instructions::init_sla_registry::handler(ctx, governance_parameters)
    }

    pub fn init_user_accounts(ctx: Context<InitUserAccounts>) -> Result<()> {
        instructions::init_user_accounts::handler(ctx)
    }
    pub fn init_provider_accounts(ctx: Context<InitProviderAccounts>) -> Result<()> {
        instructions::init_provider_accounts::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, token_amount: u64) -> Result<()> {
        instructions::stake::handler(ctx, token_amount)
    }

    pub fn validate_period(ctx: Context<ValidatePeriod>, period: u64) -> Result<()> {
        instructions::validate_period::handler(ctx, period as usize)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim::handler(ctx)
    }

    pub fn deploy_sla(
        ctx: Context<DeploySla>,
        ipfs_hash: String,
        slo: Slo,
        messenger_address: Pubkey,
        leverage: DslaDecimal,
        start: u128,
        n_periods: usize,
        period_length: PeriodLength,
    ) -> Result<()> {
        instructions::deploy_sla::handler(
            ctx,
            ipfs_hash,
            slo,
            messenger_address,
            leverage,
            start,
            n_periods,
            period_length,
        )
    }
}
