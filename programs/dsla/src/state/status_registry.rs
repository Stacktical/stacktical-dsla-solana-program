use anchor_lang::prelude::*;

/// struct used to generate the statuses for an SLA
///
///
/// # Fields
///
///  * `statuses` - the status of a given period
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
