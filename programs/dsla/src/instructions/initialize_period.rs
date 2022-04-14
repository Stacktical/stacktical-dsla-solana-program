use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::events::InitializedPeriodEvent;
use crate::state::period_generator::PeriodGenerator;

#[derive(Accounts)]
pub struct InitializePeriod<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // space: 8 discriminator + Slo max size
    #[account(
        init,
        payer = owner,
        space = 8 + PeriodGenerator::MAX_SIZE,
        seeds = [b"period_generator", owner.key().as_ref()],
        bump
    )]
    pub period_generator: Account<'info, PeriodGenerator>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializePeriod>,
    start: u128,
    period_length: u128,
    n_periods: u128,
) -> Result<()> {
    let earliest_possible_start = Clock::get()?.unix_timestamp as u128 + PeriodGenerator::MIN_DELAY;
    msg!("{}", Clock::get()?.unix_timestamp);
    require_gt!(
        start,
        earliest_possible_start,
        ErrorCode::InvalidPeriodGeneratorStart
    );

    require_gte!(
        period_length,
        PeriodGenerator::MIN_PERIOD_LENGTH,
        ErrorCode::InvalidPeriodGeneratorPeriodLength
    );

    require_gte!(n_periods, 1, ErrorCode::ZeroNumberOfPeriods);

    let period_generator = &mut ctx.accounts.period_generator;
    period_generator.start = start;
    period_generator.period_length = period_length;
    period_generator.n_periods = n_periods;
    period_generator.bump = *match ctx.bumps.get("period_generator") {
        Some(bump) => bump,
        None => return err!(ErrorCode::BumpNotFound),
    };

    emit!(InitializedPeriodEvent {
        start,
        period_length,
        n_periods,
    });
    Ok(())
}
