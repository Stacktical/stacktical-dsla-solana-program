use anchor_lang::prelude::*;

use super::DslaDecimal;
/// the registry with the stored status of each period after validation
#[account]
pub struct StatusRegistry {
    pub status_registry: Vec<Status>,
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

impl StatusRegistry {
    pub fn new_vec(n_periods: u32) -> Vec<Status> {
        vec![Status::NotVerified; n_periods as usize]
    }
}
