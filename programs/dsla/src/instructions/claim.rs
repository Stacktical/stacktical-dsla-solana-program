use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::state::reward::Reward;
use crate::state::sla::{Sla, SlaAuthority};
use crate::state::status_registry::{Status, StatusRegistry};
use crate::state::utils::Side;
use crate::state::{DslaDecimal, LastClaimedPeriod, SlaStatus, Slo};

/// Instruction to claim all rewards up to the latest available
/// eg. if current period is 5 and I have never claimed before, I will receive all rewards up to 4th period according to the status, leverage and deviation
#[derive(Accounts)]
pub struct Claim<'info> {
    /// provider or user
    #[account(mut)]
    pub claimer: Signer<'info>,

    /// the SLA
    pub sla: Account<'info, Sla>,

    #[account(
        mut,
        seeds = [sla.key().as_ref()],
        bump = sla.authority_bump_seed[0],
    )]
    pub sla_authority: Account<'info, SlaAuthority>,

    /// The token account to claimer the money in
    #[account(mut)]
    pub claimer_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub claimer_ut_account: Box<Account<'info, TokenAccount>>,

    /// The token account with the ut tokens
    #[account(mut)]
    pub claimer_dsla_account: Box<Account<'info, TokenAccount>>,

    /// The token account with pt tokens
    #[account(mut)]
    pub claimer_pt_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [USER_POOL_SEED.as_bytes(), sla.key().as_ref()],
        bump,
    )]
    pub user_pool: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [PROVIDER_POOL_SEED.as_bytes(), sla.key().as_ref()],
        bump,
    )]
    pub provider_pool: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            UT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub ut_mint: Box<Account<'info, Mint>>,

    #[account(
        seeds = [
            PT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,

    pub dsla_mint: Box<Account<'info, Mint>>,

    /// PDA with pt tokens
    #[account(
            mut,
            seeds = [
                claimer.key().as_ref(),
                PT_ACCOUNT_SEED.as_bytes(),
                sla.key().as_ref()
            ],
            bump
            )]
    pub staker_pt_account: Box<Account<'info, TokenAccount>>,

    /// PDA with ut tokens
    #[account(
                mut,
                seeds = [
                    claimer.key().as_ref(),
                    UT_ACCOUNT_SEED.as_bytes(),
                    sla.key().as_ref()
                ],
                bump
                )]
    pub staker_ut_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
        claimer.key().as_ref(),
         REWARD_SEED.as_bytes(),
         sla.key().as_ref(),
     ],
     bump,
 )]
    pub reward: Box<Account<'info, Reward>>,

    #[account(
        seeds = [STATUS_REGISTRY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub status_registry: Account<'info, StatusRegistry>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

fn apply_deviation_to_reward(slo: &Slo, reward: u64, value: DslaDecimal) -> Result<u64> {
    // @fixme find a better way to set the precison
    let precision = 18;

    Ok(Decimal::from_u64(reward)
        .unwrap()
        .checked_mul(slo.get_deviation(value, precision)?)
        .unwrap()
        .floor()
        .to_u64()
        .unwrap())
}

impl<'info> Claim<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let side = &self.reward.side;
        match side {
            Side::Provider => CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.provider_pool.to_account_info(),
                    to: self.claimer_token_account.to_account_info(),
                    authority: self.sla_authority.to_account_info(),
                },
            ),
            Side::User => CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.user_pool.to_account_info(),
                    to: self.claimer_token_account.to_account_info(),
                    authority: self.sla_authority.to_account_info(),
                },
            ),
        }
    }

    fn calculate_total_claimable_reward(&mut self) -> Result<u64> {
        let current_period_id = &self.sla.period_data.get_current_period_id()?;
        let reward = &mut self.reward;
        let status_registry = &self.status_registry.status_registry;
        let mut first_claimable_period: usize;

        let slo = &self.sla.slo;

        match reward.last_claimed_period {
            LastClaimedPeriod::NeverClaimed => first_claimable_period = 0,
            LastClaimedPeriod::Claimed {
                last_claimed_period,
            } => first_claimable_period = last_claimed_period + 1,
        };

        if first_claimable_period > (self.sla.period_data.n_periods - 1) {
            return Ok(0);
        }
        let mut total_reward = 0;

        match *current_period_id {
            SlaStatus::Active { period_id } => {
                if period_id == 0 {
                    return Ok(0);
                }

                reward.last_claimed_period = LastClaimedPeriod::Claimed {
                    last_claimed_period: (period_id - 1),
                };

                match reward.side {
                    Side::Provider => {
                        if let Status::Respected { value } = status_registry[first_claimable_period]
                        {
                            total_reward += apply_deviation_to_reward(
                                slo,
                                reward.current_period_reward,
                                value,
                            )?;
                        }
                        reward.current_period_reward = reward.future_periods_reward;

                        if first_claimable_period == (self.sla.period_data.n_periods - 1) {
                            return Ok(total_reward);
                        }

                        first_claimable_period += 1;

                        for status in &status_registry[first_claimable_period..period_id] {
                            if let Status::Respected { value } = status {
                                total_reward += apply_deviation_to_reward(
                                    slo,
                                    reward.future_periods_reward,
                                    *value,
                                )?;
                            }
                        }
                    }
                    Side::User => {
                        if let Status::NotRespected { value } =
                            status_registry[first_claimable_period]
                        {
                            total_reward += apply_deviation_to_reward(
                                slo,
                                reward.current_period_reward,
                                value,
                            )?;
                        }
                        reward.current_period_reward = reward.future_periods_reward;

                        if first_claimable_period == (self.sla.period_data.n_periods - 1) {
                            return Ok(total_reward);
                        }

                        first_claimable_period += 1;

                        for status in &status_registry[first_claimable_period..period_id] {
                            if let Status::NotRespected { value } = status {
                                total_reward += apply_deviation_to_reward(
                                    slo,
                                    reward.future_periods_reward,
                                    *value,
                                )?;
                            }
                        }
                    }
                };
            }

            SlaStatus::NotStarted => return Ok(0),
            SlaStatus::Ended => {
                let last_claimable_period = self.sla.period_data.n_periods - 1;
                reward.last_claimed_period = LastClaimedPeriod::Claimed {
                    last_claimed_period: last_claimable_period,
                };

                match reward.side {
                    Side::Provider => {
                        if let Status::Respected { value } = status_registry[first_claimable_period]
                        {
                            total_reward += apply_deviation_to_reward(
                                slo,
                                reward.current_period_reward,
                                value,
                            )?;
                        }
                        reward.current_period_reward = reward.future_periods_reward;

                        if first_claimable_period == (self.sla.period_data.n_periods - 1) {
                            return Ok(total_reward);
                        }

                        first_claimable_period += 1;

                        for status in
                            &status_registry[first_claimable_period..last_claimable_period]
                        {
                            if let Status::Respected { value } = status {
                                total_reward += apply_deviation_to_reward(
                                    slo,
                                    reward.future_periods_reward,
                                    *value,
                                )?;
                            }
                        }
                    }
                    Side::User => {
                        if let Status::NotRespected { value } =
                            status_registry[first_claimable_period]
                        {
                            total_reward += apply_deviation_to_reward(
                                slo,
                                reward.current_period_reward,
                                value,
                            )?;
                        }
                        reward.current_period_reward = reward.future_periods_reward;

                        if first_claimable_period == (self.sla.period_data.n_periods - 1) {
                            return Ok(total_reward);
                        }

                        first_claimable_period += 1;

                        for status in
                            &status_registry[first_claimable_period..last_claimable_period]
                        {
                            if let Status::NotRespected { value } = status {
                                total_reward += apply_deviation_to_reward(
                                    slo,
                                    reward.future_periods_reward,
                                    *value,
                                )?;
                            }
                        }
                    }
                };
            }
        };
        Ok(total_reward)
        // @remind think about what happens to all the non utilized funds
    }
}

pub fn handler(ctx: Context<Claim>) -> Result<()> {
    let total_claimable_reward = ctx.accounts.calculate_total_claimable_reward()?;

    // // BURN TOKENS
    // let burn_cpi_context = CpiContext::new(
    //     ctx.accounts.token_program.to_account_info(),
    //     Burn {
    //         mint: ctx.accounts.dsla_mint.to_account_info(),
    //         from: ctx.accounts.claimer_dsla_account.to_account_info(),
    //         authority: ctx.accounts.claimer.to_account_info(),
    //     },
    // );
    // // @todo set DSLA burn rate
    // let dsla_burn = 1000;
    // token::burn(burn_cpi_context, dsla_burn)?;

    // TRANSFER TOKENS
    let transfer_cpi_context = ctx.accounts.transfer_context();
    token::transfer(transfer_cpi_context, total_claimable_reward)?;

    Ok(())
}
