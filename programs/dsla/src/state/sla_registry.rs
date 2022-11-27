use anchor_lang::prelude::*;

/// the vector containing all the address for all active DSLAs
#[account]
pub struct SlaRegistry {
    pub sla_account_addresses: Vec<Pubkey>,
}
