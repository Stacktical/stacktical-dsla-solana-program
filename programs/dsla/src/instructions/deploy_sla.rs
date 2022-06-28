use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::sla::{Sla, SlaAuthority, Slo};
use crate::state::sla_registry::SlaRegistry;
use crate::state::status_registry::{Status, StatusRegistry};
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

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = deployer,
        seeds = [PROVIDER_POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint = mint,
        token::authority = sla_authority,
        bump,
    )]
    pub provider_pool: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = deployer,
        seeds = [USER_POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint = mint,
        token::authority = sla_authority,
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
    ipfs_hash: String,
    slo: Slo,
    messenger_address: Pubkey,
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
    let initial_seeds = &[sla.to_account_info().key.as_ref()];
    let (authority, authority_seed) = Pubkey::find_program_address(initial_seeds, ctx.program_id);

    sla.sla_authority = authority;
    sla.authority_seed = sla.key();
    sla.authority_bump_seed = [authority_seed];
    sla.leverage = leverage;
    sla.messenger_address = messenger_address;
    sla.ipfs_hash = ipfs_hash;
    sla.slo = slo;

    sla.mint_address = ctx.accounts.mint.key();

    // PERIOD REGISTRY
    // let period_registry = &mut ctx.accounts.period_registry;
    // require_gt!(300, periods.len());

    // for period in &periods {
    //     require!(
    //         period.status == Status::NotVerified,
    //         ErrorCode::PeriodAlreadyVerified
    //     );
    // }
    // period_registry.bump = *match ctx.bumps.get("period_registry") {
    //     Some(bump) => bump,
    //     None => return err!(ErrorCode::BumpNotFound),
    // };
    // period_registry.periods = periods;

    emit!(DeployedSlaEvent {
        sla_account_address: sla.key()
    });

    msg!("SLA deployed");
    Ok(())
}
