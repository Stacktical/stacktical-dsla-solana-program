use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("the SLA address provided does not have a Slo registered.")]
    SloNotFound,
    #[msg("precision is not divisible by 100")]
    InvalidPrecision,
}
