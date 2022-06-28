use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::state::sla::{Sla, SlaAuthority};
use crate::state::status_registry::{Status, StatusRegistry};
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

    #[account(mut)]
    pub withdrawer_ut_account: Box<Account<'info, TokenAccount>>,

    /// The token account with the ut tokens
    #[account(mut)]
    pub withdrawer_dsla_account: Box<Account<'info, TokenAccount>>,

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

    pub dsla_mint: Box<Account<'info, Mint>>,

    #[account(
        seeds = [STATUS_REGISTRY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub status_registry: Account<'info, StatusRegistry>,

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
    fn check_available_withdrawal_funds(&self, side: Side) -> bool {
        unimplemented!();
    }
}

pub fn handler(ctx: Context<Stake>, token_amount: u64, side: Side, period_id: usize) -> Result<()> {
    let status_registry = &ctx.accounts.status_registry;
    let status = &status_registry.statuses[period_id];
    match status {
        Status::Respected { value: _ } => {
            // CHECK AVAILABLE
            ctx.accounts.check_available_withdrawal_funds(side);
            // BURN TOKENS
            let burn_cpi_context = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.dsla_mint.to_account_info(),
                    from: ctx.accounts.withdrawer_dsla_account.to_account_info(),
                    authority: ctx.accounts.withdrawer.to_account_info(),
                },
            );
            token::burn(burn_cpi_context, token_amount)?;
            // TRANSFER TOKENS
            let transfer_cpi_context = ctx.accounts.transfer_context(side);
            token::transfer(transfer_cpi_context, token_amount)?;
        }
        _ => {}
    }
    Ok(())
}
