use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum ErrorCode {
    #[msg("precision is not divisible by 100")]
    InvalidPrecision = 7001,
    #[msg("period ID entered is not valid")]
    InvalidPeriodId = 7002,
    #[msg("trying to verify an already verified period")]
    AlreadyVerifiedPeriod = 7003,
    #[msg("Failed to convert to a decimal")]
    DecimalConversionError = 7004,
    #[msg("operation failed with an overflow")]
    CheckedOperationOverflow = 7005,
    #[msg("Not enough available tokens for withdrawal")]
    NoAvailableTokensForWithdrawal = 7006,
    #[msg("Cannot Stake After SLA has ended")]
    CannotStakeAfterSlaEnded = 7007,
    #[msg("Withdrawal should be at least 1")]
    WithdrawalIsZero = 7008,
    #[msg("SLA with the same address can only be initialized once")]
    SLaAlreadyInitialized = 7009,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum FeedErrorCode {
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount = 8001,
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed = 8002,
    #[msg("Switchboard feed exceeded provided confidence interval")]
    ConfidenceIntervalExceeded = 8003,
}
