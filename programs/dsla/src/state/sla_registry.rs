use anchor_lang::prelude::*;

/// the `SlaRegistry` is an account with a vector containing the public keys for all SLAs
#[account]
pub struct SlaRegistry {
    pub sla_account_addresses: Vec<Pubkey>,
}
