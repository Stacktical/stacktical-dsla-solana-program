use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::events::RegisteredSloEvent;
use crate::state::slo::{Slo, SloType};

#[derive(Accounts)]
pub struct RegisterSlo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // space: 8 discriminator + Slo max size
    #[account(
        init,
        payer = owner,
        space = 8 + Slo::MAX_SIZE,
        seeds = [b"slo", owner.key().as_ref()],
        bump
    )]
    pub slo: Account<'info, Slo>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterSlo>, slo_type: SloType, slo_value: u128) -> Result<()> {
    let slo = &mut ctx.accounts.slo;
    slo.slo_type = slo_type;
    slo.slo_value = slo_value;
    slo.bump = *match ctx.bumps.get("slo") {
        Some(bump) => bump,
        None => return err!(ErrorCode::BumpNotFound),
    };
    emit!(RegisteredSloEvent {
        slo_type,
        slo_value
    });
    Ok(())
}
