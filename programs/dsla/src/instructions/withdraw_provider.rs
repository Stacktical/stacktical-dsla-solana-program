use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::state::sla::Sla;
use crate::state::{Governance, Lockup};

/// Instruction to claim all rewards up to the latest available
/// eg. if current period is 5 and I have never claimed before, I will receive all rewards up to 4th period according to the status, leverage and deviation
#[derive(Accounts)]
pub struct WithdrawProvider<'info> {
    /// provider
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    /// the SLA
    #[account(mut)]
    pub sla: Account<'info, Sla>,

    #[account(
        mut,
        seeds = [SLA_AUTHORITY_SEED.as_bytes(),sla.key().as_ref()],
        bump,
    )]
    pub sla_authority: SystemAccount<'info>,

    /// The token account to claimer the money in
    #[account(mut, associated_token::mint=mint, associated_token::authority=withdrawer)]
    pub withdrawer_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::mint=dsla_mint, associated_token::authority=withdrawer)]
    pub withdrawer_dsla_account: Box<Account<'info, TokenAccount>>,

    /// The token account with pt tokens
    #[account(mut, associated_token::mint=pt_mint, associated_token::authority=withdrawer)]
    pub withdrawer_pt_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            withdrawer.key().as_ref(),
            LOCKUP_PROVIDER_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub pt_lockup: Box<Account<'info, Lockup>>,

    // @fixme make sure mint is same as defined in initialization
    #[account(
        constraint = mint.is_initialized == true,
        constraint = mint.key() == sla.mint_address,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint=mint,
        token::authority=sla_authority,
        bump,
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            PT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        constraint = pt_mint.is_initialized == true,
        bump,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,

    // @fixme make sure this is the DSLA mint and not something else
    #[account(constraint = dsla_mint.is_initialized == true)]
    pub dsla_mint: Box<Account<'info, Mint>>,

    #[account(
        seeds = [GOVERNANCE_SEED.as_bytes()],
        bump
    )]
    pub governance: Account<'info, Governance>,
    pub token_program: Program<'info, Token>,

    // @fixme is this needed?
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawProvider>, burn_amount: u64) -> Result<()> {
    let burn_amount_dec = Decimal::from_u64(burn_amount).unwrap();
    let provider_pool_size_dec = Decimal::from_u128(ctx.accounts.sla.provider_pool_size).unwrap();
    let pt_supply_dec = Decimal::from_u128(ctx.accounts.sla.pt_supply).unwrap();

    let lockup = &mut ctx.accounts.pt_lockup;
    let period_id = ctx.accounts.sla.period_data.get_current_period_id()?;

    lockup.update_available_tokens(period_id)?;

    // @todo check withdrawals for provider liquidity
    //

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

    // @todo add test
    sla.pt_supply = sla.pt_supply.checked_sub(burn_amount as u128).unwrap();

    lockup.withdraw(burn_amount)?;

    let transfer_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool.to_account_info(),
            to: ctx.accounts.withdrawer_token_account.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
    );
    // let provider_amount;
    // let protocol_amount;
    // let deployer_amount;

    // TRANSFER TOKENS
    token::transfer(transfer_context, tokens_to_withdraw)?;
    sla.provider_pool_size = sla
        .provider_pool_size
        .checked_sub(tokens_to_withdraw as u128)
        .unwrap();

    Ok(())
}
