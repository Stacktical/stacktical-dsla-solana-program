use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

use crate::state::period_registry::Period;
use crate::state::sla::Slo;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod dsla {
    use super::*;

    pub fn init_sla_registry(ctx: Context<InitSlaRegistry>) -> Result<()> {
        instructions::init_sla_registry::handler(ctx)
    }

    pub fn stake_provider_side(ctx: Context<StakeProviderSide>, token_amount: u64) -> Result<()> {
        instructions::stake_provider_side::handler(ctx, token_amount)
    }

    pub fn stake_user_side(ctx: Context<StakeUserSide>, token_amount: u64) -> Result<()> {
        instructions::stake_user_side::handler(ctx, token_amount)
    }

    pub fn deploy_sla(
        ctx: Context<DeploySla>,
        ipfs_hash: String,
        slo: Slo,
        messenger_address: Pubkey,
        periods: Vec<Period>,
        leverage: u64,
    ) -> Result<()> {
        instructions::deploy_sla::handler(ctx, ipfs_hash, slo, messenger_address, periods, leverage)
    }
}
