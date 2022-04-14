use crate::state::slo::SloType;

use anchor_lang::prelude::*;

#[event]
pub struct RegisteredSloEvent {
    pub slo_value: u128,
    pub slo_type: SloType,
}

#[event]
pub struct InitializedPeriodEvent {
    pub start: u128,
    pub period_length: u128,
    pub n_periods: u128,
}
