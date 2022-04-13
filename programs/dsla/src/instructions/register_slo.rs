use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::events::RegisteredSloEvent;
use crate::state::slo::{Slo, SloType};

#[derive(Accounts)]
#[instruction(sla_address: Pubkey)]
pub struct RegisterSlo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // space: 8 discriminator + Slo max size
    #[account(
        init,
        payer = owner,
        space = 8 + Slo::MAX_SIZE,
        seeds = [b"slo", owner.key().as_ref(), sla_address.as_ref()],
        bump
    )]
    pub slo: Account<'info, Slo>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterSlo>,
    sla_address: Pubkey,
    slo_type: SloType,
    slo_value: u128,
) -> Result<()> {
    let slo = &mut ctx.accounts.slo;
    slo.slo_type = slo_type;
    slo.slo_value = slo_value;
    slo.bump = *match ctx.bumps.get("slo") {
        Some(bump) => bump,
        None => return err!(ErrorCode::SloNotFound),
    };
    emit!(RegisteredSloEvent {
        sla_address,
        slo_type,
        slo_value
    });
    Ok(())
}
