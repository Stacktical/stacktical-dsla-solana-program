use anchor_lang::prelude::*;
use std::collections::HashMap;

// TODO: deal with modifier

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod slo_registry {
    use super::*;

    pub fn register_slo(
        ctx: Context<SLORegistry>,
        slo_value: u128,
        slo_type: SLOType,
        sla_address: Pubkey,
    ) -> Result<()> {
        Ok(())
    }

    pub fn is_respected(
        ctx: Context<SLORegistry>,
        value: u128,
        sla_address: Pubkey,
    ) -> Result<bool> {
        Ok(true)
    }

    pub fn get_deviation(
        ctx: Context<SLORegistry>,
        sli: u128,
        sla_address: Pubkey,
        precision: u128,
    ) -> Result<u128> {
        Ok(0)
    }
}

#[derive(Accounts)]
pub struct SLORegistry<'info> {
    #[account(mut)]
    pub slo_registry: AccountInfo<'info>,
    pub registered_slo: HashMap<Pubkey, SLO>,
    sla_registry: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum SLOType {
    EqualTo,
    NotEqualTo,
    SmallerThan,
    SmallerOrEqualTo,
    GreaterThan,
    GreaterOrEqualTo,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SLO {
    sloValue: u128,
    sloType: SLOType,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
