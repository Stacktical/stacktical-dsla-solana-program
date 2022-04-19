use crate::state::period_registry::Period;
use crate::state::slo::SloType;

use anchor_lang::prelude::*;

#[event]
pub struct RegisteredSloEvent {
    pub slo_value: u128,
    pub slo_type: SloType,
}

#[event]
pub struct InitializedPeriodRegistryEvent {
    pub periods: Vec<Period>,
}
