use anchor_lang::prelude::*;

use super::DslaDecimal;
/// the registry with the stored status of each period after validation
#[account]
pub struct StatusRegistry {
    pub status_registry: Vec<Status>,
    pub bump: u8,
}

/// Enum defining the status
#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Clone)]
pub enum Status {
    /// Period wasn't verified yet
    NotVerified,
    /// Period was respected with `value`
    Respected { value: DslaDecimal },
    /// Period wasn't respected with `value`
    NotRespected { value: DslaDecimal },
}
