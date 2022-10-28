use anchor_lang::prelude::*;

use super::DslaDecimal;
#[account]
pub struct StatusRegistry {
    pub status_registry: Vec<Status>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Clone)]
pub enum Status {
    NotVerified,
    Respected { value: DslaDecimal },
    NotRespected { value: DslaDecimal },
}
