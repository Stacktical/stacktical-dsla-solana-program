use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::state::sla::{Sla, SlaAuthority};
use crate::state::Lockup;

/// Instruction to claim all rewards up to the latest available
/// eg. if current period is 5 and I have never claimed before, I will receive all rewards up to 4th period according to the status, leverage and deviation
#[derive(Accounts)]
pub struct WithdrawUser<'info> {
    /// user
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
    #[account(mut, token::mint=mint, token::authority=withdrawer)]
    pub withdrawer_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, token::mint=dsla_mint, token::authority=withdrawer)]
    pub withdrawer_dsla_account: Box<Account<'info, TokenAccount>>,

    /// The token account with ut tokens
    #[account(mut, token::mint=ut_mint, token::authority=withdrawer)]
    pub withdrawer_ut_account: Box<Account<'info, TokenAccount>>,

    // @fixme make sure mint is same as defined in initialization
    #[account(constraint = mint.is_initialized == true)]
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
            UT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        constraint = ut_mint.is_initialized == true,
        bump,
    )]
    pub ut_mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [
            withdrawer.key().as_ref(),
            LOCKUP_USER_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub ut_lockup: Box<Account<'info, Lockup>>,

    // @fixme make sure this is the DSLA mint and not something else
    #[account(constraint = dsla_mint.is_initialized == true)]
    pub dsla_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawUser>, burn_amount: u64) -> Result<()> {
    // @todo add DSLA burn
    let burn_amount_dec = Decimal::from_u64(burn_amount).unwrap();
    let user_pool_size_dec = Decimal::from_u128(ctx.accounts.sla.user_pool_size).unwrap();
    let ut_supply_dec = Decimal::from_u128(ctx.accounts.sla.ut_supply).unwrap();
    let period_id = ctx.accounts.sla.period_data.get_current_period_id()?;

    let sla = &mut ctx.accounts.sla;

    let lockup = &mut ctx.accounts.ut_lockup;
    lockup.update_available_tokens(period_id)?;

    // @todo add test
    let tokens_to_withdraw = burn_amount_dec
        .checked_div(user_pool_size_dec.checked_div(ut_supply_dec).unwrap())
        .unwrap()
        .floor()
        .to_u64()
        .unwrap();

    // @todo add test
    // BURN TOKENS
    let burn_cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.ut_mint.to_account_info(),
            from: ctx.accounts.withdrawer_ut_account.to_account_info(),
            authority: ctx.accounts.withdrawer.to_account_info(),
        },
    );
    token::burn(burn_cpi_context, burn_amount)?;
    sla.ut_supply -= burn_amount as u128;

    lockup.withdraw(burn_amount)?;

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
    sla.user_pool_size -= tokens_to_withdraw as u128;

    Ok(())
}
