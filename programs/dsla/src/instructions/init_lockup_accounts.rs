use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::{Lockup, Sla};

/// Instruction to initialize the SLARegistry
#[derive(Accounts)]
pub struct InitLockupAccounts<'info> {
    #[account(mut)]
    pub user_provider: Signer<'info>,

    #[account()]
    pub sla: Account<'info, Sla>,

    #[account(
        init,
        space = Lockup::LEN,
        payer = user_provider,
        seeds = [
            user_provider.key().as_ref(),
            LOCKUP_PROVIDER_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub pt_lockup: Box<Account<'info, Lockup>>,

    #[account(
        init,
        space = Lockup::LEN,
        payer = user_provider,
        seeds = [
            user_provider.key().as_ref(),
            LOCKUP_USER_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub ut_lockup: Box<Account<'info, Lockup>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitLockupAccounts>) -> Result<()> {
    ctx.accounts.pt_lockup.set_inner(Lockup::new());
    ctx.accounts.ut_lockup.set_inner(Lockup::new());
    Ok(())
}
