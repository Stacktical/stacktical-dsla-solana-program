use anchor_lang::prelude::*;

use crate::state::sla_registry::SlaRegistry;

#[derive(Accounts)]
pub struct InitSlaRegistry<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,
    #[account(zero)]
    pub sla_registry: Account<'info, SlaRegistry>,
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<InitSlaRegistry>) -> Result<()> {
    msg!("SLA registry Initialized");
    Ok(())
}
