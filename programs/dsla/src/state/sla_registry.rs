use anchor_lang::prelude::*;

#[account]
pub struct SlaRegistry {
    pub sla_account_addresses: Vec<Pubkey>,
}
