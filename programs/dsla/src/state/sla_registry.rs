use anchor_lang::prelude::*;

#[account]
pub struct SlaRegistry {
    slas: Vec<Pubkey>,
}

impl SlaRegistry {}
