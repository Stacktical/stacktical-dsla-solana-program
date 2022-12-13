use anchor_lang::prelude::*;
use switchboard_v2::{AggregatorAccountData, SWITCHBOARD_PROGRAM_ID};

use crate::constants::*;
use crate::errors::{ErrorCode, FeedErrorCode};
use crate::events::*;
use crate::state::sla::{PeriodGenerator, PeriodLength};
use crate::state::sla::{Sla, Slo};
use crate::state::sla_registry::SlaRegistry;
use crate::state::status_registry::StatusRegistry;
use crate::state::{DslaDecimal, Governance, SlaAuthority};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

/// Instruction to deploy a new SLA
#[derive(Accounts)]
pub struct DeploySla<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,

    #[account(mut)]
    pub sla_registry: Account<'info, SlaRegistry>,

    #[account(
        init,
        payer = deployer,
        space = Sla::LEN,
    )]
    pub sla: Account<'info, Sla>,

    #[account(
        init,
        payer = deployer,
        space = 8,
        seeds = [SLA_AUTHORITY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub sla_authority: Account<'info, SlaAuthority>,

    #[account(
        init,
        payer = deployer,
        space = 10_000, // @fixme set the correct size depending on n_periods
        seeds = [STATUS_REGISTRY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub status_registry: Account<'info, StatusRegistry>,

    #[account(constraint = mint.is_initialized == true)]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = deployer,
        seeds = [POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint = mint,
        token::authority = sla_authority,
        bump,
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    // @todo add constraint to check for correct DSLA mint address
    #[account(constraint = dsla_mint.is_initialized == true)]
    pub dsla_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = deployer,
        seeds = [DSLA_POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint = dsla_mint,
        token::authority = sla_authority,
        bump,
    )]
    pub dsla_pool: Box<Account<'info, TokenAccount>>,

    /// The token account to pay the DSLA fee from
    #[account(mut, associated_token::mint=dsla_mint, associated_token::authority=deployer)]
    pub deployer_dsla_token_account: Box<Account<'info, TokenAccount>>,

    // keep this here to check that governance account has been initialized before deploying an SLA
    #[account(
        seeds = [GOVERNANCE_SEED.as_bytes()],
        bump
    )]
    pub governance: Account<'info, Governance>,
    #[account(
        init,
        payer = deployer,
        seeds = [
            UT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        // @fixme check that this actually work
        mint::decimals = mint.decimals,
        mint::authority = sla_authority,
        bump,
    )]
    pub ut_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = deployer,
        seeds = [
            PT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        mint::decimals = mint.decimals,
        mint::authority = sla_authority,
        bump
    )]
    pub pt_mint: Box<Account<'info, Mint>>,

    #[account(
        constraint = *aggregator.to_account_info().owner == SWITCHBOARD_PROGRAM_ID @ FeedErrorCode::InvalidSwitchboardAccount
    )]
    pub aggregator: AccountLoader<'info, AggregatorAccountData>,

    /// The program for interacting with the token.
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

impl<'info> DeploySla<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.deployer_dsla_token_account.to_account_info(),
                to: self.dsla_pool.to_account_info(),
                authority: self.deployer.to_account_info(),
            },
        )
    }
}
pub fn handler(
    ctx: Context<DeploySla>,
    slo: Slo,
    leverage: DslaDecimal,
    start: u128,
    n_periods: u32,
    period_length: PeriodLength,
) -> Result<()> {
    // check that the SLA registry still has space
    // @todo add error for this
    require_gt!(
        312499,
        ctx.accounts.sla_registry.sla_account_addresses.len()
    );
    // @todo add error for this
    require_gte!(9, ctx.accounts.mint.decimals);
    // @todo add test for this
    require!(
        !ctx.accounts
            .sla_registry
            .sla_account_addresses
            .contains(&ctx.accounts.sla.key()),
        ErrorCode::SLaAlreadyInitialized
    );

    ctx.accounts
        .sla_registry
        .sla_account_addresses
        .push(ctx.accounts.sla.key());
    msg!("{}", ctx.accounts.sla_registry.sla_account_addresses[0]);

    let transfer_amount = ctx
        .accounts
        .governance
        .dsla_deposit_by_period
        .checked_mul(n_periods as u64)
        .unwrap();
    token::transfer(ctx.accounts.transfer_context(), transfer_amount)?;
    let sla = &mut ctx.accounts.sla;

    // SLA initialization
    sla.leverage = leverage;
    sla.provider_pool_size = 0;
    sla.user_pool_size = 0;
    sla.slo = slo;
    sla.period_data = PeriodGenerator::new(start, period_length, n_periods);
    sla.mint_address = ctx.accounts.mint.key();
    sla.sla_deployer_address = ctx.accounts.deployer.key();
    sla.aggregator_address = ctx.accounts.aggregator.key();

    // Status registry initialization
    ctx.accounts.status_registry.status_registry = StatusRegistry::new_vec(n_periods);

    emit!(DeployedSlaEvent {
        sla_account_address: sla.key()
    });

    Ok(())
}
