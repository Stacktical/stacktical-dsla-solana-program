use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::state::sla::{Sla, SlaAuthority};

/// Instruction to claim all rewards up to the latest available
/// eg. if current period is 5 and I have never claimed before, I will receive all rewards up to 4th period according to the status, leverage and deviation
#[derive(Accounts)]
pub struct WithdrawProvider<'info> {
    /// provider
    #[account(mut)]
    pub withdrawer: Signer<'info>,

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
    pub withdrawer_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub withdrawer_dsla_account: Box<Account<'info, TokenAccount>>,

    /// The token account with pt tokens
    #[account(mut)]
    pub withdrawer_pt_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [POOL_SEED.as_bytes(), sla.key().as_ref()],
        bump,
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            PT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,

    pub dsla_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawProvider>, burn_amount: u64) -> Result<()> {
    // @todo add DSLA burn
    let burn_amount_dec = Decimal::from_u64(burn_amount).unwrap();
    let provider_pool_size_dec = Decimal::from_u128(ctx.accounts.sla.provider_pool_size).unwrap();
    let pt_supply_dec = Decimal::from_u128(ctx.accounts.sla.pt_supply).unwrap();

    // @todo ad test
    let tokens_to_withdraw = burn_amount_dec
        .checked_div(provider_pool_size_dec.checked_div(pt_supply_dec).unwrap())
        .unwrap()
        .floor()
        .to_u64()
        .unwrap();

    // @todo add test
    // BURN TOKENS
    let burn_cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.pt_mint.to_account_info(),
            from: ctx.accounts.withdrawer_pt_account.to_account_info(),
            authority: ctx.accounts.withdrawer.to_account_info(),
        },
    );
    token::burn(burn_cpi_context, burn_amount)?;
    let sla = &mut ctx.accounts.sla;

    sla.pt_supply -= burn_amount as u128;

    let transfer_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool.to_account_info(),
            to: ctx.accounts.withdrawer_token_account.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
    );
    // TRANSFER TOKENS
    let transfer_cpi_context = transfer_context;
    token::transfer(transfer_cpi_context, tokens_to_withdraw)?;
    sla.provider_pool_size -= tokens_to_withdraw as u128;

    Ok(())
}
