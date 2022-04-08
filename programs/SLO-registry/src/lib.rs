use anchor_lang::prelude::*;
use std::collections::HashMap;

// FIXME: deal with modifier only SLARegistry can call the functions

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod slo_registry {
    use super::*;

    // FIXME: function should be called only once on deployment
    pub fn set_sla_registry(ctx: Context<Registry>, sla_registry: Pubkey) -> Result<()> {
        ctx.accounts.slo_registry.sla_registry = sla_registry;
        ctx.accounts.slo_registry.registered_slo = HashMap::new();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Registry<'info> {
    #[account(init, payer = owner, space = 1024)]
    pub slo_registry: Account<'info, SLORegistry>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct SLORegistry {
    pub registered_slo: HashMap<Pubkey, SLO>,
    sla_registry: Pubkey,
}

impl SLORegistry {
    pub fn register_slo(&mut self, sla_address: Pubkey, slo: SLO) -> Result<()> {
        self.registered_slo.insert(sla_address, slo);
        Ok(())
    }

    pub fn is_respected(&self, value: u128, sla_address: Pubkey) -> Result<bool> {
        let registered_slo = match self.registered_slo.get(&sla_address) {
            Some(map) => map,
            None => return err!(ErrorCode::SLONotFound),
        };

        let slo_type = registered_slo.slo_type;
        let slo_value = registered_slo.slo_value;

        match slo_type {
            SLOType::EqualTo => Ok(value == slo_value),
            SLOType::NotEqualTo => Ok(value != slo_value),
            SLOType::SmallerThan => Ok(value < slo_value),
            SLOType::SmallerOrEqualTo => Ok(value <= slo_value),
            SLOType::GreaterThan => Ok(value > slo_value),
            SLOType::GreaterOrEqualTo => Ok(value >= slo_value),
        }
    }

    pub fn get_deviation(&self, sli: u128, sla_address: Pubkey, precision: u128) -> Result<u128> {
        if precision % 100 != 0 {
            return err!(ErrorCode::InvalidPrecision);
        }

        let registered_slo = match self.registered_slo.get(&sla_address) {
            Some(map) => map,
            None => return err!(ErrorCode::SLONotFound),
        };

        let slo_type = registered_slo.slo_type;
        let slo_value = registered_slo.slo_value;

        let mut deviation: u128 = (if sli >= slo_value {
            sli - slo_value
        } else {
            slo_value
        }) * precision
            / ((sli + slo_value) / 2);

        if deviation > (precision * 25 / 100) {
            deviation = precision * 25 / 100;
        }
        match slo_type {
            // Deviation of 1%
            SLOType::EqualTo | SLOType::NotEqualTo => Ok(precision / 100),
            _ => Ok(deviation),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
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
    pub slo_value: u128,
    pub slo_type: SLOType,
}

impl SLO {
    pub fn new(slo_value: u128, slo_type: SLOType) -> Self {
        Self {
            slo_value,
            slo_type,
        }
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("the SLA address provided does not have a SLO registered.")]
    SLONotFound,
    #[msg("precision is not divisible by 100")]
    InvalidPrecision,
}
