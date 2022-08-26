use crate::state::utils::Decimal;
use anchor_lang::prelude::*;
#[account]
pub struct StatusRegistry {
    pub statuses: Vec<Status>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Clone)]
pub enum Status {
    NotVerified,
    Respected { value: Decimal },
    NotRespected { value: Decimal },
}
