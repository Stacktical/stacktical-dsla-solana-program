use anchor_lang::prelude::*;

use crate::state::governance::Governance;
use crate::state::sla_registry::SlaRegistry;
use crate::utils::*;

#[derive(Accounts)]
pub struct InitSlaRegistry<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,
    #[account(zero)]
    pub sla_registry: Account<'info, SlaRegistry>,
    #[account(
        init,
        payer = deployer,
        space = Governance::LEN,
        seeds = [GOVERNANCE_SEED.as_bytes()],
        bump
    )]
    pub governance: Account<'info, Governance>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitSlaRegistry>, governance_parameters: Governance) -> Result<()> {
    let governance = &mut ctx.accounts.governance;
    governance.dsla_burn_rate = governance_parameters.dsla_burn_rate;
    governance.dsla_deposit_by_period = governance_parameters.dsla_deposit_by_period;
    governance.dsla_platform_reward = governance_parameters.dsla_platform_reward;
    governance.dsla_messenger_reward = governance_parameters.dsla_messenger_reward;
    governance.dsla_user_reward = governance_parameters.dsla_user_reward;
    governance.dsla_burned_by_verification = governance_parameters.dsla_burned_by_verification;
    governance.max_token_length = governance_parameters.max_token_length;
    governance.max_leverage = governance_parameters.max_leverage;
    governance.burn_dsla = governance_parameters.burn_dsla;

    msg!("SLA registry Initialized");
    Ok(())
}
