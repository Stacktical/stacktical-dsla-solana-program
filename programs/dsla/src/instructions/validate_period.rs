use crate::constants::*;
use crate::errors::{ErrorCode, FeedErrorCode};
use crate::program::Dsla;
use crate::state::sla::{DslaDecimal, Sla, Slo};
use crate::state::status_registry::{Status, StatusRegistry};
use crate::state::Governance;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;
use switchboard_v2::{AggregatorAccountData, SwitchboardDecimal, SWITCHBOARD_PROGRAM_ID};

/// Instruction to validate a period x, anyone can validate
#[derive(Accounts)]
pub struct ValidatePeriod<'info> {
    #[account(mut)]
    pub validator: Signer<'info>,
    #[account(
        mut,
        seeds = [SLA_AUTHORITY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub sla_authority: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [STATUS_REGISTRY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub status_registry: Account<'info, StatusRegistry>,

    #[account(mut)]
    pub sla: Account<'info, Sla>,

    #[account(
        constraint = *aggregator.to_account_info().owner == SWITCHBOARD_PROGRAM_ID @ FeedErrorCode::InvalidSwitchboardAccount,
        constraint = aggregator.key() == sla.aggregator_address
    )]
    pub aggregator: AccountLoader<'info, AggregatorAccountData>,

    #[account(
        seeds = [GOVERNANCE_SEED.as_bytes()],
        bump
    )]
    pub governance: Account<'info, Governance>,
    // @todo add constraint to check for correct DSLA mint address
    #[account(constraint = dsla_mint.is_initialized == true)]
    pub dsla_mint: Box<Account<'info, Mint>>,
    #[account(
            seeds = [DSLA_POOL_SEED.as_bytes(), sla.key().as_ref()],
            token::mint = dsla_mint,
            token::authority = sla_authority,
            bump,
        )]
    pub dsla_pool: Box<Account<'info, TokenAccount>>,

    /// The validator token account to pay the DSLA reward to
    #[account(mut, associated_token::mint=dsla_mint, associated_token::authority=validator)]
    pub validator_dsla_token_account: Box<Account<'info, TokenAccount>>,

    #[account(address = crate::ID)]
    pub program: Program<'info, Dsla>,
    // @fixme this need to be checked, that only allowed program_data is the one linked to the program
    pub program_data: Account<'info, ProgramData>,

    /// CHECK: constraint is checking that this account has the right public key
    #[account(constraint = program_data.upgrade_authority_address == Some(protocol.key()))]
    pub protocol: AccountInfo<'info>,

    #[account(
        associated_token::mint = dsla_mint,
        associated_token::authority = program_data.upgrade_authority_address.unwrap()
    )]
    pub protocol_token_account: Box<Account<'info, TokenAccount>>,
    /// The program for interacting with the token.
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}
impl<'info> ValidatePeriod<'info> {
    fn validator_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.dsla_pool.to_account_info(),
                to: self.validator_dsla_token_account.to_account_info(),
                authority: self.sla_authority.to_account_info(),
            },
        )
    }
    fn protocol_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.dsla_pool.to_account_info(),
                to: self.protocol_token_account.to_account_info(),
                authority: self.protocol.to_account_info(),
            },
        )
    }
}

fn get_reward(slo: &Slo, sli: DslaDecimal, periods_left: u64, pool_size: u128) -> Result<u64> {
    // @todo find a better way to set the precison
    let precision = 18;
    let reward = Decimal::from_u128(pool_size)
        .unwrap()
        .checked_div(Decimal::from_u64(periods_left).unwrap())
        .unwrap();

    Ok(reward
        .checked_mul(slo.get_deviation(sli, precision)?)
        .unwrap()
        .floor()
        .to_u64()
        .unwrap())
}

pub fn handler(ctx: Context<ValidatePeriod>, period: usize) -> Result<()> {
    let status_registry = &mut ctx.accounts.status_registry.status_registry;
    let slo = ctx.accounts.sla.slo.clone();

    match status_registry[period] {
        Status::NotVerified => {
            let max_confidence_interval = Some(100.0); // @todo change this to a protocol governance const or sla level const
            let max_staleness = 300; // @todo change this to a protocol governance variable or sla level variable

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
            let sla = &mut ctx.accounts.sla;
            let periods_left = status_registry.len().checked_sub(period).unwrap();

            if respected {
                let reward = get_reward(
                    &slo,
                    sli_dsla_decimal,
                    periods_left as u64,
                    sla.provider_pool_size,
                )?;
                sla.provider_pool_size =
                    sla.provider_pool_size.checked_sub(reward as u128).unwrap();
                sla.user_pool_size = sla.user_pool_size.checked_add(reward as u128).unwrap();
                status_registry[period] = Status::Respected {
                    value: sli_dsla_decimal,
                };
            } else {
                let reward = get_reward(
                    &slo,
                    sli_dsla_decimal,
                    periods_left as u64,
                    sla.user_pool_size,
                )?;
                sla.user_pool_size = sla.user_pool_size.checked_sub(reward as u128).unwrap();
                sla.provider_pool_size =
                    sla.provider_pool_size.checked_add(reward as u128).unwrap();
                status_registry[period] = Status::NotRespected {
                    value: sli_dsla_decimal,
                };
            }

            // @todo 4. REWARD VALIDATOR
            let sla_key = sla.key().clone();
            let authority_bump = *ctx
                .bumps
                .get("sla_authority")
                .expect("sla_authority should exists");
            let seeds = &[
                SLA_AUTHORITY_SEED.as_bytes(),
                sla_key.as_ref(),
                &[authority_bump],
            ];
            let signer_seeds = &[&seeds[..]];

            let burn_context = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.dsla_mint.to_account_info(),
                    from: ctx.accounts.dsla_pool.to_account_info(),
                    authority: ctx.accounts.sla_authority.to_account_info(),
                },
                signer_seeds,
            );

            token::transfer(
                ctx.accounts.protocol_transfer_context(),
                ctx.accounts.governance.dsla_protocol_reward,
            )?;
            token::transfer(
                ctx.accounts.validator_transfer_context(),
                ctx.accounts.governance.dsla_validator_reward,
            )?;
            token::burn(
                burn_context,
                ctx.accounts.governance.dsla_burned_by_verification,
            )?;

            Ok(())
        }
        _ => err!(ErrorCode::AlreadyVerifiedPeriod),
    }
}
