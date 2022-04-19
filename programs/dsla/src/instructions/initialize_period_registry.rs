use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::events::InitializedPeriodRegistryEvent;
use crate::state::period_registry::Period;
use crate::state::period_registry::PeriodRegistry;

#[derive(Accounts)]
#[instruction(periods: Vec<u128>)]
pub struct InitializePeriodRegistry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // space: 8 discriminator +
    #[account(
        init,
        payer = owner,
        space = 8 + 4 + (periods.len() * Period::MAX_SIZE),
        seeds = [b"period_registry", owner.key().as_ref()],
        bump
    )]
    pub period_registry: Account<'info, PeriodRegistry>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializePeriodRegistry>, periods: Vec<Period>) -> Result<()> {
    let leftover_space = 10_000 - 8 - 4 - (periods.len() * 4);
    require_gte!(leftover_space, 0_usize);
    let earliest_possible_start = Clock::get()?.unix_timestamp as u64 + PeriodRegistry::MIN_DELAY;
    require_gte!(
        periods[0].start,
        earliest_possible_start,
        ErrorCode::InvalidPeriodStart
    );
    require_gt!(periods.len(), 0_usize, ErrorCode::ZeroNumberOfPeriods);
    require!(
        PeriodRegistry::verify_period_length(&periods),
        ErrorCode::InvalidPeriodLength
    );

    let period_registry = &mut ctx.accounts.period_registry;
    period_registry.periods = periods.clone();
    period_registry.bump = *match ctx.bumps.get("period_registry") {
        Some(bump) => bump,
        None => return err!(ErrorCode::BumpNotFound),
    };

    emit!(InitializedPeriodRegistryEvent { periods });
    Ok(())
}
