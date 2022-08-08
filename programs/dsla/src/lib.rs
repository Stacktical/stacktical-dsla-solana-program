use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

use crate::state::governance::Governance;
use crate::state::sla::Slo;
use crate::state::utils::Side;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod dsla {
    use super::*;

    pub fn init_sla_registry(
        ctx: Context<InitSlaRegistry>,
        governance_parameters: Governance,
    ) -> Result<()> {
        instructions::init_sla_registry::handler(ctx, governance_parameters)
    }

    pub fn init_ut_pt_accounts(ctx: Context<InitUtPtAccounts>) -> Result<()> {
        instructions::init_ut_pt_accounts::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, token_amount: u64, side: Side) -> Result<()> {
        instructions::stake::handler(ctx, token_amount, side)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        token_amount: u64,
        side: Side,
        period_id: u32,
    ) -> Result<()> {
        instructions::withdraw::handler(ctx, token_amount, side, period_id as usize)
    }

    pub fn deploy_sla(
        ctx: Context<DeploySla>,
        ipfs_hash: String,
        slo: Slo,
        messenger_address: Pubkey,
        leverage: u64,
    ) -> Result<()> {
        instructions::deploy_sla::handler(ctx, ipfs_hash, slo, messenger_address, leverage)
    }
}
