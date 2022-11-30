use crate::constants::*;
use crate::program::Dsla;
use crate::state::governance::Governance;
use anchor_lang::prelude::*;

/// Instruction to initialize the SLARegistry
#[derive(Accounts)]
pub struct InitGovernance<'info> {
    /// the account that has the authority to upgrade the program
    #[account(mut)]
    pub program_upgrade_authority: Signer<'info>,
    #[account(
        init,
        payer = program_upgrade_authority,
        space = Governance::LEN,
        seeds = [GOVERNANCE_SEED.as_bytes()],
        bump
    )]
    pub governance: Account<'info, Governance>,
    #[account(constraint = program.programdata_address()? == Some(program_upgrade_authority.key()))]
    pub program: Program<'info, Dsla>,
    #[account(constraint = program_data.upgrade_authority_address == Some(program_upgrade_authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitGovernance>, governance_parameters: Governance) -> Result<()> {
    ctx.accounts.governance.set_inner(governance_parameters);

    msg!("Governance Initialised successfully");
    Ok(())
}
