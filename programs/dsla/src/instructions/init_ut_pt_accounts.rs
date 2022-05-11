use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::sla::Sla;
use crate::utils::*;

#[derive(Accounts)]
pub struct InitUtPtAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub sla: Account<'info, Sla>,

    /// init the token account with the ut tokens
    #[account(
        init,
        payer = signer,
        seeds = [
            signer.key().as_ref(),
            UT_ACCOUNT_SEED.as_bytes(),
            sla.key().as_ref()
        ],
        token::mint = ut_mint,
        token::authority = signer,
        bump
    )]
    pub staker_ut_account: Box<Account<'info, TokenAccount>>,

    /// init the token account with pt tokens
    #[account(
        init,
        payer = signer,
        seeds = [
            signer.key().as_ref(),
            PT_ACCOUNT_SEED.as_bytes(),
            sla.key().as_ref()
        ],
        token::mint = pt_mint,
        token::authority = signer,
        bump
    )]
    pub staker_pt_account: Box<Account<'info, TokenAccount>>,

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

pub fn handler(_ctx: Context<InitUtPtAccounts>) -> Result<()> {
    Ok(())
}
