use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum ErrorCode {
    #[msg("precision is not divisible by 100")]
    InvalidPrecision, // 6000
    #[msg("period ID entered is not valid")]
    InvalidPeriodId, // 6001
    #[msg("trying to verify an already verified period")]
    AlreadyVerifiedPeriod, // 6002
    #[msg("Failed to convert to a decimal")]
    DecimalConversionError, // 6003
    #[msg("operation failed with an overflow")]
    CheckedOperationOverflow, // 6004
    #[msg("Not enough available tokens for withdrawal")]
    NoAvailableTokensForWithdrawal, // 6005
    #[msg("Cannot Stake After SLA has ended")]
    CannotStakeAfterSlaEnded, // 6006
    #[msg("Withdrawal should be at least 1")]
    WithdrawalIsZero, // 6007
    #[msg("SLA with the same address can only be initialized once")]
    SLaAlreadyInitialized, // 6008
    #[msg("1 or more non Valid governance Parameters")]
    NonValidGovernanceParameters, // 6009
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum FeedErrorCode {
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount,
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed,
    #[msg("Switchboard feed exceeded provided confidence interval")]
    ConfidenceIntervalExceeded,
}
