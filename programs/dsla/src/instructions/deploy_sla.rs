use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::period_registry::{Period, PeriodRegistry};
use crate::state::sla::{Sla, Slo};
use crate::state::sla_registry::SlaRegistry;

#[derive(Accounts)]
pub struct DeploySla<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,
    pub sla_registry: Account<'info, SlaRegistry>,
    #[account(
        init,
        payer = deployer,
        space = 10_000
    )]
    pub sla: Account<'info, Sla>,
    #[account(
        init,
        payer = deployer,
        space = Sla::MAX_SIZE,
        seeds = [b"period-registry", sla.key().to_bytes().as_ref()],
        bump
    )]
    pub period_registry: Account<'info, PeriodRegistry>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<DeploySla>,
    ipfs_hash: String,
    // slo: Slo,
    messenger_address: Pubkey,
    periods: Vec<Period>,
    leverage: u64,
) -> Result<()> {
    let sla_registry = &mut ctx.accounts.sla_registry;
    let sla = &mut ctx.accounts.sla;
    let period_registry = &mut ctx.accounts.period_registry;

    // SLA REGISTRY
    // check that SLA registry still has space
    require_gt!(312499, sla_registry.sla_account_addresses.len());
    sla_registry.sla_account_addresses.push(sla.key());

    // SLA initialization
    sla.leverage = leverage;
    sla.messenger_address = messenger_address;
    sla.ipfs_hash = ipfs_hash;
    // sla.slo = slo;

    // PERIOD REGISTRY
    require_gt!(300, periods.len());
    period_registry.bump = *match ctx.bumps.get("period-registry") {
        Some(bump) => bump,
        None => return err!(ErrorCode::BumpNotFound),
    };
    period_registry.periods = periods;
    emit!(InitializedPeriodRegistryEvent {
        periods: period_registry.periods.clone()
    });

    emit!(CreatedSlaEvent {
        sla_account_address: sla.key()
    });
    msg!("SLA Created");
    Ok(())
}
