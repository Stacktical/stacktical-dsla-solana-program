use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::period_registry::{Period, PeriodRegistry, Status};
use crate::state::sla::{Sla, Slo};
use crate::state::sla_registry::SlaRegistry;
use crate::utils::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

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
        space = 10_000,
        seeds = [PERIOD_REGISTRY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub period_registry: Account<'info, PeriodRegistry>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = deployer,
        seeds = [PROVIDER_POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint = mint,
        token::authority = sla,
        bump,
    )]
    pub provider_pool: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = deployer,
        seeds = [USER_POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint = mint,
        token::authority = sla,
        bump,
    )]
    pub user_pool: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = deployer,
        seeds = [
            UT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        mint::decimals = 6,
        mint::authority = sla,
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
        mint::decimals = 6,
        mint::authority = sla,
        bump
    )]
    pub pt_mint: Box<Account<'info, Mint>>,

    /// The program for interacting with the token.
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<DeploySla>,
    ipfs_hash: String,
    slo: Slo,
    messenger_address: Pubkey,
    periods: Vec<Period>,
    leverage: u64,
) -> Result<()> {
    let sla = &mut ctx.accounts.sla;

    // SLA REGISTRY
    let sla_registry = &mut ctx.accounts.sla_registry;

    // check that SLA registry still has space
    require_gt!(312499, sla_registry.sla_account_addresses.len());
    sla_registry.sla_account_addresses.push(sla.key());
    msg!("{}", sla_registry.sla_account_addresses[0]);

    // SLA initialization
    sla.leverage = leverage;
    sla.messenger_address = messenger_address;
    sla.ipfs_hash = ipfs_hash;
    sla.slo = slo;

    sla.mint_address = ctx.accounts.mint.key();

    // PERIOD REGISTRY
    let period_registry = &mut ctx.accounts.period_registry;
    require_gt!(300, periods.len());

    for period in &periods {
        require!(
            period.status == Status::NotVerified,
            ErrorCode::PeriodAlreadyVerified
        );
    }
    period_registry.bump = *match ctx.bumps.get("period_registry") {
        Some(bump) => bump,
        None => return err!(ErrorCode::BumpNotFound),
    };
    period_registry.periods = periods;

    emit!(DeployedSlaEvent {
        sla_account_address: sla.key()
    });

    msg!("SLA deployed");
    Ok(())
}
