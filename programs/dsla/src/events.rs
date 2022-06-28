use crate::state::status_registry::Status;

use anchor_lang::prelude::*;

#[event]
pub struct InitializedStatusRegistryEvent {
    pub periods: Vec<Status>,
}

#[event]
pub struct DeployedSlaEvent {
    pub sla_account_address: Pubkey,
}

#[event]
pub struct InitializedSlaRegistryEvent {
    pub sla_addresses: Vec<Pubkey>,
}
#[event]
pub struct StakedProviderSideEvent {
    pub token_amount: u64,
}

#[event]
pub struct StakedUserSideEvent {
    pub token_amount: u64,
}
