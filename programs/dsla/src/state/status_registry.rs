use anchor_lang::prelude::*;

use crate::errors::ErrorCode;

/// struct used to generate the periods for an SLA
///
///
/// # Fields
///
///  * `periods` - the timestamps of each step
///
#[account]
pub struct StatusRegistry {
    pub statuses: Vec<Status>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Clone)]
pub enum Status {
    NotVerified,
    Respected { value: u128 },
    NotRespected { value: u128 },
}
