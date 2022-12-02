use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::events::StakedUserSideEvent;
use crate::state::sla::Sla;
use crate::state::Lockup;

/// Instruction to stake on both sides
#[derive(Accounts)]
pub struct StakeUser<'info> {
    // provide or user
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(mut)]
    pub sla: Account<'info, Sla>,

    #[account(
        mut,
        seeds = [SLA_AUTHORITY_SEED.as_bytes(),sla.key().as_ref()],
        bump,
    )]
    pub sla_authority: SystemAccount<'info>,

    // @fixme make sure mint is same as defined in initialization
    #[account(
        constraint = mint.is_initialized == true,
        constraint = mint.key() == sla.mint_address,
)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint=mint,
        token::authority=sla_authority,
        seeds = [POOL_SEED.as_bytes(), sla.key().as_ref()],
        bump,
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            UT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        constraint = ut_mint.is_initialized == true,
        bump,
    )]
    pub ut_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            staker.key().as_ref(),
            LOCKUP_USER_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub ut_lockup: Box<Account<'info, Lockup>>,

    /// The account to claim the money from
    #[account(mut, associated_token::mint=mint, associated_token::authority=staker)]
    pub staker_token_account: Box<Account<'info, TokenAccount>>,

    /// ut tokens
    #[account(mut, associated_token::mint=ut_mint, associated_token::authority=staker)]
    pub staker_ut_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakeUser<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.staker_token_account.to_account_info(),
                to: self.pool.to_account_info(),
                authority: self.staker.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<StakeUser>, token_amount: u64) -> Result<()> {
    let leverage = &ctx.accounts.sla.leverage.to_decimal();
    let token_amount_dec = Decimal::from_u64(token_amount).unwrap();
    let provider_pool_size_dec = Decimal::from_u128(ctx.accounts.sla.provider_pool_size).unwrap();
    let user_pool_size_dec = Decimal::from_u128(ctx.accounts.sla.user_pool_size).unwrap();
    let ut_supply_dec = Decimal::from_u128(ctx.accounts.sla.ut_supply).unwrap();

    // @todo add test for this
    let leverage_adjusted_user_pool = leverage
        .checked_mul(user_pool_size_dec.checked_add(token_amount_dec).unwrap())
        .unwrap();

    // @todo add test and error for this
    require_gte!(provider_pool_size_dec, leverage_adjusted_user_pool);

    let mut tokens_to_mint = token_amount;
    // @todo add test for this
    if user_pool_size_dec != ut_supply_dec {
        // @todo add test for this
        tokens_to_mint = token_amount_dec
            .checked_div(user_pool_size_dec.checked_div(ut_supply_dec).unwrap())
            .unwrap()
            .floor()
            .to_u64()
            .unwrap();
    }

    token::transfer(ctx.accounts.transfer_context(), token_amount)?;
    let sla = &mut ctx.accounts.sla;

    // @todo add test for this
    sla.user_pool_size = sla
        .user_pool_size
        .checked_add(token_amount as u128)
        .unwrap();

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

    let mint_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            to: ctx.accounts.staker_ut_account.to_account_info(),
            mint: ctx.accounts.ut_mint.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
        signer_seeds,
    );

    token::mint_to(mint_context, tokens_to_mint)?;
    // @todo add test for this
    sla.ut_supply = sla.ut_supply.checked_add(tokens_to_mint as u128).unwrap();

    let lockup = &mut ctx.accounts.ut_lockup;
    let period_id = ctx.accounts.sla.period_data.get_current_period_id()?;

    lockup.stake_update(tokens_to_mint, period_id)?;

    // @todo improve this event
    emit!(StakedUserSideEvent { token_amount });
    Ok(())
}
