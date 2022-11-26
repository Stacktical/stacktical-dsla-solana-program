use anchor_lang::prelude::*;

use crate::constants::*;
use crate::events::*;
use crate::state::sla::{PeriodGenerator, PeriodLength};
use crate::state::sla::{Sla, SlaAuthority, Slo};
use crate::state::sla_registry::SlaRegistry;
use crate::state::status_registry::StatusRegistry;
use crate::state::DslaDecimal;
use anchor_spl::token::{Mint, Token, TokenAccount};

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
        seeds = [sla.key().as_ref()],
        bump
    )]
    pub sla_authority: Account<'info, SlaAuthority>,

    #[account(
        init,
        payer = deployer,
        space = 10_000,
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

    #[account(
        init,
        payer = deployer,
        seeds = [
            UT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        mint::decimals = 6,
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
        mint::decimals = 6,
        mint::authority = sla_authority,
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
    slo: Slo,
    messenger_address: Pubkey,
    leverage: DslaDecimal,
    start: u128,
    n_periods: u32,
    period_length: PeriodLength,
) -> Result<()> {
    let sla = &mut ctx.accounts.sla;

    // SLA REGISTRY
    let sla_registry = &mut ctx.accounts.sla_registry;

    // check that the SLA registry still has space
    require_gt!(312499, sla_registry.sla_account_addresses.len());
    sla_registry.sla_account_addresses.push(sla.key());
    msg!("{}", sla_registry.sla_account_addresses[0]);

    // SLA initialization
    let initial_seeds = &[sla.to_account_info().key.as_ref()];
    let (authority, authority_seed) = Pubkey::find_program_address(initial_seeds, ctx.program_id);

    sla.sla_authority = authority;
    sla.authority_seed = sla.key();
    sla.authority_bump_seed = [authority_seed];
    sla.leverage = leverage;
    sla.messenger_address = messenger_address;
    sla.provider_pool_size = 0;
    sla.user_pool_size = 0;
    sla.slo = slo;
    sla.period_data = PeriodGenerator::new(start, period_length, n_periods);
    sla.mint_address = ctx.accounts.mint.key();

    emit!(DeployedSlaEvent {
        sla_account_address: sla.key()
    });

    msg!("SLA deployed");
    Ok(())
}
