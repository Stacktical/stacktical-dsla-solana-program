use anchor_lang::prelude::*;

#[account]
pub struct SlaAuthority {}

impl SlaAuthority {
    pub const LEN: usize = 8; // discriminator
}
