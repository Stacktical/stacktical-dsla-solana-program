use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::events::StakedUserSideEvent;
use crate::state::reward::Reward;
use crate::state::sla::{Sla, SlaAuthority};

use crate::state::utils::Side;
#[derive(Accounts)]
pub struct Stake<'info> {
    // provide or user
    #[account(mut)]
    pub staker: Signer<'info>,
    pub sla: Account<'info, Sla>,

    #[account(
        mut,
        seeds = [sla.key().as_ref()],
        bump = sla.authority_bump_seed[0],
    )]
    pub sla_authority: Account<'info, SlaAuthority>,

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

    /// PDA with the reward
    #[account(
        seeds = [
            REWARD_SEED.as_bytes(),
            sla.key().as_ref(),
            staker.key().as_ref()
        ],
        bump,
    )]
    pub reward: Box<Account<'info, Reward>>,

    /// The account to withdraw the money from
    pub staker_token_account: Box<Account<'info, TokenAccount>>,

    /// PDA with pt tokens
    #[account(
        mut,
        seeds = [
            staker.key().as_ref(),
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
                staker.key().as_ref(),
                UT_ACCOUNT_SEED.as_bytes(),
                sla.key().as_ref()
            ],
            bump
            )]
    pub staker_ut_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    fn transfer_context(&self, side: Side) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        match side {
            Side::Provider => CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.staker_token_account.to_account_info(),
                    to: self.provider_pool.to_account_info(),
                    authority: self.staker.to_account_info(),
                },
            ),
            Side::User => CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.staker_token_account.to_account_info(),
                    to: self.user_pool.to_account_info(),
                    authority: self.staker.to_account_info(),
                },
            ),
        }
    }
}

pub fn handler(ctx: Context<Stake>, token_amount: u64, side: Side) -> Result<()> {
    let max_reward = ctx
        .accounts
        .sla
        .leverage
        .to_decimal()
        .checked_mul(Decimal::from_u64(token_amount).unwrap())
        .unwrap()
        .to_u128()
        .unwrap();

    if let Side::User = side {
        require_gte!(ctx.accounts.sla.total_liquidity_available, max_reward);
    };

    // GET CURRENT PERIOD ID
    let current_period_id = ctx.accounts.sla.period_data.get_current_period_id()?;
    // CHECK THAT THIS ISN'T THE LAST PERIOD
    require_neq!(
        current_period_id,
        ctx.accounts.sla.period_data.n_periods - 1
    );

    // CALCULATE NEW REWARD FOR REMAINING PERIODS

    token::transfer(ctx.accounts.transfer_context(side), token_amount)?;
    ctx.accounts.sla.total_liquidity_available =
        ctx.accounts.sla.total_liquidity_available - max_reward;
    let auth_seed = ctx.accounts.sla.authority_seed;
    let seeds = &[auth_seed.as_ref(), &ctx.accounts.sla.authority_bump_seed];
    let signer = &[&seeds[..]];

    let cpi_accounts = match side {
        Side::Provider => MintTo {
            to: ctx.accounts.staker_pt_account.to_account_info(),
            mint: ctx.accounts.pt_mint.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
        Side::User => MintTo {
            to: ctx.accounts.staker_ut_account.to_account_info(),
            mint: ctx.accounts.ut_mint.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
    };

    let mint_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );

    token::mint_to(mint_context, token_amount)?;

    emit!(StakedUserSideEvent { token_amount });
    Ok(())
}
