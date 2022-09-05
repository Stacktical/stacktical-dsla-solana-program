use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
pub use switchboard_v2::{AggregatorAccountData, SwitchboardDecimal, SWITCHBOARD_PROGRAM_ID};

use crate::constants::*;
use crate::state::sla::Sla;
use crate::state::status_registry::StatusRegistry;
use crate::state::utils::Decimal;
#[derive(Accounts)]
pub struct ValidatePeriod<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [STATUS_REGISTRY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub status_registry: Account<'info, StatusRegistry>,
    #[account(
        constraint = *aggregator.to_account_info().owner == SWITCHBOARD_PROGRAM_ID @ FeedErrorCode::InvalidSwitchboardAccount
    )]
    pub aggregator: AccountLoader<'info, AggregatorAccountData>,
    pub sla: Account<'info, Sla>,
}

pub fn handler(ctx: Context<ValidatePeriod>, _period: u128, _sli: Decimal) -> Result<()> {
    let max_confidence_interval = Some(10.0);
    let _slo = &ctx.accounts.sla.slo;
    // TODO: once the period is expired allow the validation using a stream with unlimited time horizon 0.5% get_sli somehow;

    // 1. get the data

    let feed = &ctx.accounts.aggregator.load()?;

    // get result
    let val: f64 = feed.get_result()?.try_into()?;

    // check whether the feed has been updated in the last 300 seconds
    feed.check_staleness(clock::Clock::get().unwrap().unix_timestamp, 300)
        .map_err(|_| error!(FeedErrorCode::StaleFeed))?;

    // check feed does not exceed max_confidence_interval
    if let Some(max_confidence_interval) = max_confidence_interval {
        feed.check_confidence_interval(SwitchboardDecimal::from_f64(max_confidence_interval))
            .map_err(|_| error!(FeedErrorCode::ConfidenceIntervalExceeded))?;
    }

    msg!("Current feed result is {}!", val);

    // 2. compare slo to sli
    // 3. update status
    // 4. reward validator

    Ok(())
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum FeedErrorCode {
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount,
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed,
    #[msg("Switchboard feed exceeded provided confidence interval")]
    ConfidenceIntervalExceeded,
}
