use crate::constants::*;
use crate::errors::ErrorCode;
// use crate::program::Dsla;
use crate::state::governance::Governance;
use anchor_lang::prelude::*;

/// Instruction to initialize the SLARegistry
#[derive(Accounts)]
pub struct ModifyGovernance<'info> {
    /// the account that has the authority to upgrade the program
    #[account(mut)]
    pub program_upgrade_authority: Signer<'info>,
    #[account(mut,
        seeds = [GOVERNANCE_SEED.as_bytes()],
        bump
    )]
    pub governance: Account<'info, Governance>,
    // #[account(constraint = program.programdata_address()? == Some(program_upgrade_authority.key()))]
    // pub program: Program<'info, Dsla>,
    // #[account(constraint = program_data.upgrade_authority_address == Some(program_upgrade_authority.key()))]
    // pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ModifyGovernance>, governance_parameters: Governance) -> Result<()> {
    require!(
        governance_parameters.dsla_deposit_by_period
            == (governance_parameters.dsla_burned_by_verification
                + governance_parameters.dsla_validator_reward
                + governance_parameters.dsla_protocol_reward),
        ErrorCode::NonValidGovernanceParameters
    );
    ctx.accounts.governance.set_inner(governance_parameters);

    msg!("Governance modified successfully");
    Ok(())
}