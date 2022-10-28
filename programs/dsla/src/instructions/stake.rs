use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::events::StakedUserSideEvent;
use crate::state::reward::Reward;
use crate::state::sla::{Sla, SlaAuthority, SlaStatus};

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

    /// The account to claim the money from
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
    #[account(
           seeds = [
            staker.key().as_ref(),
            REWARD_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub reward: Box<Account<'info, Reward>>,

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

pub fn handler(ctx: Context<Stake>, token_amount: u64) -> Result<()> {
    let leverage = &ctx.accounts.sla.leverage.to_decimal();
    let token_amount_dec = Decimal::from_u64(token_amount).unwrap();
    let max_reward = leverage
        .checked_mul(token_amount_dec)
        .unwrap()
        .to_u128()
        .unwrap();

    if let Side::User = &ctx.accounts.reward.side {
        require_gte!(ctx.accounts.sla.total_liquidity_available, max_reward);
    };

    // GET CURRENT PERIOD ID
    // @remind check that is not returning 0 on the last period is left
    let current_period_id = &ctx.accounts.sla.period_data.get_current_period_id()?;
    let reward = &mut ctx.accounts.reward;
    // @remind double check this calculations
    // @remind since we are using floor some small portion of tokens will be left in the pool at the end of all SLAs
    match current_period_id {
        SlaStatus::NotStarted => {
            let periods_left = Decimal::from_usize(ctx.accounts.sla.period_data.n_periods).unwrap();
            let added_reward = token_amount_dec
                .checked_div(periods_left)
                .unwrap()
                .checked_mul(*leverage)
                .unwrap()
                .floor()
                .to_u64()
                .unwrap();
            reward.current_period_reward += added_reward;
            reward.future_periods_reward += added_reward;
        }
        SlaStatus::Ended => return err!(ErrorCode::StakingWindowClosed),
        SlaStatus::Active { period_id } => {
            if *period_id >= (ctx.accounts.sla.period_data.n_periods - 1) {
                return err!(ErrorCode::StakingWindowClosed);
            } else {
                let periods_left =
                    Decimal::from_usize(ctx.accounts.sla.period_data.n_periods - (*period_id + 1))
                        .unwrap();
                let added_reward = token_amount_dec
                    .checked_div(periods_left)
                    .unwrap()
                    .checked_mul(*leverage)
                    .unwrap()
                    .floor()
                    .to_u64()
                    .unwrap();
                reward.future_periods_reward = added_reward;
            }
        }
    }

    token::transfer(
        ctx.accounts.transfer_context(ctx.accounts.reward.side),
        token_amount,
    )?;
    ctx.accounts.sla.total_liquidity_available -= max_reward;
    let auth_seed = ctx.accounts.sla.authority_seed;
    let seeds = &[auth_seed.as_ref(), &ctx.accounts.sla.authority_bump_seed];
    let signer = &[&seeds[..]];

    let cpi_accounts = match &ctx.accounts.reward.side {
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
