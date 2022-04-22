use crate::state::period_registry::Period;

use anchor_lang::prelude::*;

#[event]
pub struct InitializedPeriodRegistryEvent {
    pub periods: Vec<Period>,
}

#[event]
pub struct CreatedSlaEvent {
    pub sla_account_address: Pubkey,
}
