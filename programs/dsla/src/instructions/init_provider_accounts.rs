use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;
use crate::state::sla::Sla;

/// Instruction to initialize all the accounts from provider side for an SLA
#[derive(Accounts)]
pub struct InitProviderAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub sla: Account<'info, Sla>,

    /// init the token account with pt tokens
    #[account(
        init,
        payer = signer,
        token::mint = pt_mint,
        token::authority = signer,
    )]
    pub staker_pt_account: Box<Account<'info, TokenAccount>>,

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

pub fn handler(_ctx: Context<InitProviderAccounts>) -> Result<()> {
    Ok(())
}
