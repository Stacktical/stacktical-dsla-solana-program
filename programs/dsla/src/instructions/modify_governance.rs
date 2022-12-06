use crate::constants::*;
use crate::errors::ErrorCode;
use crate::program::Dsla;
use crate::state::governance::Governance;
use crate::state::DslaDecimal;
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
    #[account(address = crate::ID)]
    pub program: Program<'info, Dsla>,
    #[account(constraint = program_data.upgrade_authority_address == Some(program_upgrade_authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ModifyGovernance>,
    dsla_deposit_by_period: u64,
    dsla_protocol_reward: u64,
    dsla_validator_reward: u64,
    dsla_burned_by_verification: u64,
    sla_deployer_rewards_rate: DslaDecimal,
    protocol_rewards_rate: DslaDecimal,
    max_leverage: DslaDecimal,
) -> Result<()> {
    require!(
        dsla_deposit_by_period
            == (dsla_burned_by_verification + dsla_validator_reward + dsla_protocol_reward),
        ErrorCode::NonValidGovernanceParameters
    );
    let governance = &mut ctx.accounts.governance;

    governance.dsla_deposit_by_period = dsla_deposit_by_period;
    governance.dsla_protocol_reward = dsla_protocol_reward;
    governance.dsla_validator_reward = dsla_validator_reward;
    governance.dsla_burned_by_verification = dsla_burned_by_verification;
    governance.sla_deployer_rewards_rate = sla_deployer_rewards_rate;
    governance.protocol_rewards_rate = protocol_rewards_rate;
    governance.max_leverage = max_leverage;

    msg!("Governance Initialised successfully");
    Ok(())
}
