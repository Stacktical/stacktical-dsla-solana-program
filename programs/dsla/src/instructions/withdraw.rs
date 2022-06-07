use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::state::period_registry::{Period, PeriodRegistry, Status};
use crate::state::sla::{Sla, SlaAuthority};
use crate::state::utils::Side;

#[derive(Accounts)]
pub struct Stake<'info> {
    // provide or user
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    pub sla: Account<'info, Sla>,

    #[account(
        mut,
        seeds = [sla.key().as_ref()],
        bump = sla.authority_bump_seed[0],
    )]
    pub sla_authority: Account<'info, SlaAuthority>,

    /// The token account to withdraw the money in
    #[account(mut)]
    pub withdrawer_token_account: Box<Account<'info, TokenAccount>>,

    /// The token account with the ut tokens
    #[account(mut)]
    pub withdrawer_ut_account: Box<Account<'info, TokenAccount>>,

    /// The token account with pt tokens
    #[account(mut)]
    pub withdrawer_pt_account: Box<Account<'info, TokenAccount>>,

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

    #[account(
        seeds = [PERIOD_REGISTRY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub period_registry: Account<'info, PeriodRegistry>,

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
                    from: self.provider_pool.to_account_info(),
                    to: self.withdrawer_token_account.to_account_info(),
                    authority: self.sla_authority.to_account_info(),
                },
            ),
            Side::User => CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.user_pool.to_account_info(),
                    to: self.withdrawer_token_account.to_account_info(),
                    authority: self.sla_authority.to_account_info(),
                },
            ),
        }
    }
}

pub fn handler(ctx: Context<Stake>, token_amount: u64, side: Side, periodId: usize) -> Result<()> {
    let period_registry = &ctx.accounts.period_registry;
    let status = &period_registry.periods[periodId].status;
    match status {
        Status::Respected { value: _ } => {
            // CHECK AVAILABLE
            // BURN TOKENS
            // TRANSFER TOKENS
        }
        _ => {}
    }
    Ok(())
}
