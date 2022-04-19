use anchor_lang::prelude::*;

#[account]
pub struct Sla {
    sla_name: String,
    ipfs_hash: String,
    messenger_address: Pubkey,
}

impl Sla {}
