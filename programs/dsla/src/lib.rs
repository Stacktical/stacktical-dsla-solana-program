use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

use crate::state::sla::Slo;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod dsla {
    use super::*;

    pub fn create_sla(
        ctx: Context<CreateSla>,
        ipfs_hash: String,
        slo: Slo,
        messenger_address: Pubkey,
        periods: Vec<(u64, u64)>,
        leverage: u64,
    ) -> Result<()> {
        instructions::create_sla::handler(ctx, ipfs_hash, slo, messenger_address, periods, leverage)
    }
}
