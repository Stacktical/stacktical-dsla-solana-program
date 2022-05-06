use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

use crate::events::StakedUserSideEvent;
use crate::state::sla::Sla;
use crate::utils::*;

#[derive(Accounts)]
pub struct Stake<'info> {
    // provide or user
    #[account(mut)]
    pub authority: Signer<'info>,
    pub sla: Account<'info, Sla>,

    /// The token account with the tokens to be staked
    #[account(mut)]
    pub staker: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [USER_POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint = sla.mint_address,
        token::authority = sla,
        bump,
    )]
    pub user_pool: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [PROVIDER_POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint = sla.mint_address,
        token::authority = sla,
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

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.staker.to_account_info(),
                to: self.user_pool.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
    fn mint_context(&self, side: Side) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        match side {
            Side::Provider => CpiContext::new(
                self.token_program.to_account_info(),
                MintTo {
                    to: self.staker.to_account_info(),
                    mint: self.pt_mint.to_account_info(),
                    authority: self.authority.to_account_info(),
                },
            ),
            Side::User => CpiContext::new(
                self.token_program.to_account_info(),
                MintTo {
                    to: self.staker.to_account_info(),
                    mint: self.ut_mint.to_account_info(),
                    authority: self.authority.to_account_info(),
                },
            ),
        }
    }
}

pub fn handler(ctx: Context<Stake>, token_amount: u64, side: Side) -> Result<()> {
    let user_pool_amount = ctx.accounts.user_pool.amount;
    let provider_pool_amount = ctx.accounts.provider_pool.amount;
    if let Side::User = side {
        require_gte!(provider_pool_amount, user_pool_amount + token_amount);
    }

    token::transfer(ctx.accounts.transfer_context(), token_amount)?;

    token::mint_to(ctx.accounts.mint_context(side), token_amount)?;

    emit!(StakedUserSideEvent { token_amount });
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Clone)]
pub enum Side {
    Provider,
    User,
}
