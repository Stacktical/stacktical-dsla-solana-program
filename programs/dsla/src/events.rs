use crate::state::slo::SloType;

use anchor_lang::prelude::*;

#[event]
pub struct RegisteredSloEvent {
    pub sla_address: Pubkey,
    pub slo_value: u128,
    pub slo_type: SloType,
}
