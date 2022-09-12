use anchor_lang::prelude::Result;
use anchor_lang::prelude::*;
use rust_decimal::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

use crate::state::governance::Governance;
use crate::state::sla::Slo;
use crate::state::utils::Side;
use crate::state::SloType;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod dsla {
    use state::SloType;

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

    pub fn validate_period(ctx: Context<ValidatePeriod>, period: u64) -> Result<()> {
        instructions::validate_period::handler(ctx, period as usize)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        token_amount: u64,
        side: Side,
        period_id: u64,
    ) -> Result<()> {
        instructions::withdraw::handler(ctx, token_amount, side, period_id as usize)
    }

    pub fn deploy_sla(
        ctx: Context<DeploySla>,
        ipfs_hash: String,
        slo_num: i64,
        slo_scale: u32,
        slo_type: SloType,
        messenger_address: Pubkey,
        leverage: u64,
    ) -> Result<()> {
        let slo = Slo {
            slo_value: Decimal::new(slo_num, slo_scale),
            slo_type,
        };
        instructions::deploy_sla::handler(ctx, ipfs_hash, slo, messenger_address, leverage)
    }
}
