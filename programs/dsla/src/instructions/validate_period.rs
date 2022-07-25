use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::sla::Sla;
use crate::state::status_registry::StatusRegistry;
use crate::state::utils::Decimal;
#[derive(Accounts)]
pub struct ValidatePeriod<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [STATUS_REGISTRY_SEED.as_bytes(), sla.key().as_ref()],
        bump
    )]
    pub status_registry: Account<'info, StatusRegistry>,
    pub sla: Account<'info, Sla>,
}

pub fn handler(ctx: Context<ValidatePeriod>, _period: u128, _sli: Decimal) -> Result<()> {
    let _slo = &ctx.accounts.sla.slo;
    // TODO: get_sli somehow;
    unimplemented!()
}
