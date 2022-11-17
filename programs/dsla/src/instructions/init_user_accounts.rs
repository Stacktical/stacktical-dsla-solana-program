use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;
use crate::state::sla::Sla;

/// Instruction to initialize all the accounts from user side for an SLA
#[derive(Accounts)]
pub struct InitUserAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub sla: Account<'info, Sla>,

    /// init the token account with the ut tokens
    #[account(
        init,
        payer = signer,
        token::mint = ut_mint,
        token::authority = signer,
    )]
    pub staker_ut_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            UT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub ut_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<InitUserAccounts>) -> Result<()> {
    Ok(())
}
