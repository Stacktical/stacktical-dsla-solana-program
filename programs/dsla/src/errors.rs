use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Could not find a bump for this key.")]
    BumpNotFound = 1,
    #[msg("precision is not divisible by 100")]
    InvalidPrecision = 2,
    #[msg("period ID entered is not valid")]
    InvalidPeriodId = 3,
    #[msg("the start is too close")]
    InvalidPeriodStart = 4,
    #[msg("the period lenght is too short")]
    InvalidPeriodLength = 5,
    #[msg("Number of periods cannot be 0")]
    ZeroNumberOfPeriods = 6,
    #[msg("Number of periods is capped based on account storage requirment")]
    MaxNumberOfPeriods = 7,
    #[msg("all periods should be set as unverified")]
    PeriodAlreadyVerified = 8,
    #[msg("decimals is different")]
    Differentdecimals = 9,
}
