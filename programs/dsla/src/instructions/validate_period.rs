use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use rust_decimal::prelude::*;
pub use switchboard_v2::{AggregatorAccountData, SwitchboardDecimal, SWITCHBOARD_PROGRAM_ID};

use crate::constants::*;
use crate::errors::{ErrorCode, FeedErrorCode};
use crate::state::sla::{DslaDecimal, Sla};
use crate::state::status_registry::{Status, StatusRegistry};

/// Instruction to validate a period x, anyone can validate
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
    pub validator: Signer<'info>,
}

pub fn handler(ctx: Context<ValidatePeriod>, period: usize) -> Result<()> {
    let status_registry = &mut ctx.accounts.status_registry.status_registry;
    let slo = &ctx.accounts.sla.slo;

    match status_registry[period] {
        Status::NotVerified => {
            let max_confidence_interval = Some(100.0); // @remind change this to a protocol governance const or sla level const
            let max_staleness = 300; // @remind change this to a protocol governance variable or sla level variable

            // @todo once the period is expired allow the validation using a stream with unlimited time horizon 0.5% get_sli somehow;
            // @todo add checks for correct datafeed account based on SLA governance variable

            // 1. GET THE DATA

            let feed = &ctx.accounts.aggregator.load()?;

            // get result
            let data: f64 = feed.get_result()?.try_into()?;
            let sli_decimal = Decimal::from_f64(data).ok_or(ErrorCode::DecimalConversionError)?;
            let sli_dsla_decimal = DslaDecimal::from_decimal(sli_decimal);

            // check whether the feed has been updated in the last max_staleness seconds
            feed.check_staleness(clock::Clock::get().unwrap().unix_timestamp, max_staleness)
                .map_err(|_| error!(FeedErrorCode::StaleFeed))?;

            // check feed does not exceed max_confidence_interval
            if let Some(max_confidence_interval) = max_confidence_interval {
                feed.check_confidence_interval(SwitchboardDecimal::from_f64(
                    max_confidence_interval,
                ))
                .map_err(|_| error!(FeedErrorCode::ConfidenceIntervalExceeded))?;
            }

            // 2. COMPARE SLO TO SLI
            let respected = slo.is_respected(sli_dsla_decimal)?;

            // 3. UPDATE STATUS
            if respected {
                status_registry[period] = Status::Respected {
                    value: sli_dsla_decimal,
                };
            } else {
                status_registry[period] = Status::NotRespected {
                    value: sli_dsla_decimal,
                };
            }

            // @todo 4. REWARD VALIDATOR

            Ok(())
        }
        _ => err!(ErrorCode::AlreadyVerifiedPeriod),
    }
}
