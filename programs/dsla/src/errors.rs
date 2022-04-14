use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Could not find a bump for this key.")]
    BumpNotFound,
    #[msg("precision is not divisible by 100")]
    InvalidPrecision,
    #[msg("period ID entered is not valid")]
    InvalidPeriodId,
    #[msg("the start is too close")]
    InvalidPeriodGeneratorStart,
    #[msg("the period lenght is too short")]
    InvalidPeriodGeneratorPeriodLength,
    #[msg("Number of periods cannot be 0")]
    ZeroNumberOfPeriods,
}
