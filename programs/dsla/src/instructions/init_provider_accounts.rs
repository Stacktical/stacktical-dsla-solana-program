use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;
use crate::state::reward::{LastClaimedPeriod, Reward};
use crate::state::sla::Sla;
use crate::state::Side;

#[derive(Accounts)]
pub struct InitProviderAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub sla: Account<'info, Sla>,

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
        init,
        payer = signer,
        seeds = [
            signer.key().as_ref(),
            REWARD_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        space = Reward::LEN,
        bump,
    )]
    pub reward: Box<Account<'info, Reward>>,

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

pub fn handler(ctx: Context<InitProviderAccounts>) -> Result<()> {
    let reward = &mut ctx.accounts.reward;
    reward.current_period_reward = 0;
    reward.future_periods_reward = 0;
    reward.last_claimed_period = LastClaimedPeriod::NeverClaimed;
    reward.side = Side::Provider;
    Ok(())
}
