use anchor_lang::prelude::*;

/// the side of the stake
#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Side {
    Provider,
    User,
}
