use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum ErrorCode {
    #[msg("Could not find a bump for this key.")]
    BumpNotFound = 1001,
    #[msg("precision is not divisible by 100")]
    InvalidPrecision = 1002,
    #[msg("period ID entered is not valid")]
    InvalidPeriodId = 1003,
    #[msg("the start is too close")]
    InvalidPeriodStart = 1004,
    #[msg("the period lenght is too short")]
    InvalidPeriodLength = 1005,
    #[msg("Number of periods cannot be 0")]
    ZeroNumberOfPeriods = 1006,
    #[msg("Number of periods is capped based on account storage requirment")]
    MaxNumberOfPeriods = 1007,
    #[msg("all periods should be set as unverified")]
    PeriodAlreadyVerified = 1008,
    #[msg("decimals is different")]
    DifferentDecimals = 1009,
    #[msg("trying to verify an already verified period")]
    AlreadyVerifiedPeriod = 1010,
    #[msg("Failed to convert to a decimal")]
    DecimalConversionError = 1011,
    #[msg("operation failed with an overflow")]
    CheckedOperationOverflow = 1012,
    #[msg("The staking windows has closed")]
    StakingWindowClosed = 1013,
    #[msg("The claiming window is closed")]
    ClaimingWindowClosed = 1014,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum FeedErrorCode {
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount = 2001,
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed = 2002,
    #[msg("Switchboard feed exceeded provided confidence interval")]
    ConfidenceIntervalExceeded = 2003,
}
