use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum ErrorCode {
    #[msg("precision is not divisible by 100")]
    InvalidPrecision = 1001,
    #[msg("period ID entered is not valid")]
    InvalidPeriodId = 1002,
    #[msg("trying to verify an already verified period")]
    AlreadyVerifiedPeriod = 1003,
    #[msg("Failed to convert to a decimal")]
    DecimalConversionError = 1004,
    #[msg("operation failed with an overflow")]
    CheckedOperationOverflow = 1005,
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
