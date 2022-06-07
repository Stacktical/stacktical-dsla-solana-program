use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Copy, Clone)]
pub enum Side {
    Provider,
    User,
}
