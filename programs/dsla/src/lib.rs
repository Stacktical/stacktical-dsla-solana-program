use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod dsla {
    use super::*;

    pub fn register_slo(
        ctx: Context<RegisterSlo>,
        slo_type: SloType,
        slo_value: u128,
    ) -> Result<()> {
        instructions::register_slo::handler(ctx, slo_type, slo_value)
    }

    pub fn initialize_period_registry(
        ctx: Context<InitializePeriodRegistry>,
        periods: Vec<Period>,
    ) -> Result<()> {
        instructions::initialize_period_registry::handler(ctx, periods)
    }
}
