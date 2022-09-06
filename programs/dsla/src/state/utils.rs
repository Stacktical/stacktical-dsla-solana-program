use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Side {
    Provider,
    User,
}
