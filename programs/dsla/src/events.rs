use crate::state::period_registry::Period;

use anchor_lang::prelude::*;

#[event]
pub struct InitializedPeriodRegistryEvent {
    pub periods: Vec<Period>,
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
