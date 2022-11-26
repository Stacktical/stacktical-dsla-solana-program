use crate::errors::ErrorCode;
use crate::state::sla::SlaStatus;
use anchor_lang::prelude::*;
use std::cmp::Ordering;

/// account to keep track of tokens that need to be locked
/// rule should be if tokens was staked in the last period should stay staked for at least one full period
#[account]
pub struct Lockup {
    pub available_tokens: u64,
    locked_tokens_prev: u64,
    locked_tokens: u64,
    locked_from_period_id: u64,
}
impl Default for Lockup {
    fn default() -> Self {
        Self::new()
    }
}

impl Lockup {
    pub const LEN: usize = 8 + 16 + 16 + 16 + 16;
    pub fn new() -> Self {
        Self {
            available_tokens: 0,
            locked_tokens_prev: 0,
            locked_tokens: 0,
            locked_from_period_id: 0,
        }
    }

    pub fn update_available_tokens(&mut self, status: SlaStatus) -> Result<()> {
        match status {
            SlaStatus::NotStarted => {}
            SlaStatus::Active { period_id } => {
                let period_id = period_id as u64;

                match period_id.cmp(&(self.locked_from_period_id.checked_add(1).unwrap())) {
                    Ordering::Greater => {
                        self.available_tokens = self
                            .available_tokens
                            .checked_add(
                                self.locked_tokens_prev
                                    .checked_add(self.locked_tokens)
                                    .unwrap(),
                            )
                            .unwrap();
                        self.locked_tokens_prev = 0;
                        self.locked_tokens = 0;
                        self.locked_from_period_id = period_id;
                    }
                    Ordering::Less => {}
                    Ordering::Equal => {
                        self.available_tokens = self
                            .available_tokens
                            .checked_add(self.locked_tokens_prev)
                            .unwrap();
                        self.locked_tokens_prev = self.locked_tokens;
                        self.locked_tokens = 0;
                        self.locked_from_period_id = period_id;
                    }
                }
            }
            SlaStatus::Ended => {
                self.available_tokens = self
                    .available_tokens
                    .checked_add(
                        self.locked_tokens_prev
                            .checked_add(self.locked_tokens)
                            .unwrap(),
                    )
                    .unwrap();
                self.locked_tokens = 0;
                self.locked_tokens_prev = 0;
                self.locked_from_period_id = 0;
            }
        }
        Ok(())
    }

    pub fn stake_update(&mut self, stake_size: u64, status: SlaStatus) -> Result<()> {
        match status {
            SlaStatus::NotStarted => {
                self.locked_tokens_prev = self.locked_tokens_prev.checked_add(stake_size).unwrap();
                self.locked_from_period_id = 0;
            }
            SlaStatus::Active { period_id } => {
                let period_id = period_id as u64;
                if self.locked_from_period_id == period_id {
                    self.locked_tokens = self.locked_tokens.checked_add(stake_size).unwrap();
                }
                match period_id.cmp(&(self.locked_from_period_id.checked_add(1).unwrap())) {
                    Ordering::Greater => {
                        self.available_tokens = self
                            .available_tokens
                            .checked_add(
                                self.locked_tokens_prev
                                    .checked_add(self.locked_tokens)
                                    .unwrap(),
                            )
                            .unwrap();
                        self.locked_tokens_prev = 0;
                        self.locked_tokens = stake_size;
                        self.locked_from_period_id = period_id;
                    }
                    Ordering::Less => {}
                    Ordering::Equal => {
                        self.available_tokens = self
                            .available_tokens
                            .checked_add(self.locked_tokens_prev)
                            .unwrap();
                        self.locked_tokens_prev = self.locked_tokens;
                        self.locked_tokens = stake_size;
                        self.locked_from_period_id = period_id;
                    }
                }
            }

            SlaStatus::Ended => {
                return err!(ErrorCode::CannotStakeAfterSlaEnded);
            }
        }
        Ok(())
    }

    pub fn withdraw(&mut self, withdraw_size: u64) -> Result<()> {
        if withdraw_size < 1 {
            return err!(ErrorCode::WithdrawalIsZero);
        }
        if self.available_tokens < withdraw_size {
            return err!(ErrorCode::NoAvailableTokensForWithdrawal);
        }
        self.available_tokens = self.available_tokens.checked_sub(withdraw_size).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lockup_struct() {
        let lockup = Lockup::new();
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_current_period() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 1 };

        lockup.stake_update(1000, status).unwrap();
        assert_eq!(lockup.available_tokens, 0);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 1000);
        assert_eq!(lockup.locked_from_period_id, 1);
        assert_eq!(
            lockup.withdraw(50),
            err!(ErrorCode::NoAvailableTokensForWithdrawal)
        );
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_period_after() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 1 };

        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 2 };
        lockup.update_available_tokens(status).unwrap();

        assert_eq!(lockup.available_tokens, 0);
        assert_eq!(lockup.locked_tokens_prev, 1000);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 2);
        assert_eq!(
            lockup.withdraw(50),
            err!(ErrorCode::NoAvailableTokensForWithdrawal)
        );
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_2_periods_after() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 1 };

        lockup.stake_update(1000, status).unwrap();

        let status = SlaStatus::Active { period_id: 3 };
        lockup.update_available_tokens(status).unwrap();

        assert_eq!(lockup.available_tokens, 1000);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 3);
        assert_eq!(lockup.withdraw(50), Ok(()));
        assert_eq!(lockup.available_tokens, 950);
    }

    #[test]
    fn check_lots_periods_after() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 1 };

        lockup.stake_update(1000, status).unwrap();

        let status = SlaStatus::Active { period_id: 120 };
        lockup.update_available_tokens(status).unwrap();

        assert_eq!(lockup.available_tokens, 1000);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 120);
        assert_eq!(lockup.withdraw(1000), Ok(()));
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_current_period_multiple_stakes() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 1 };

        lockup.stake_update(1000, status).unwrap();
        lockup.stake_update(1000, status).unwrap();
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 0);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 3000);
        assert_eq!(lockup.locked_from_period_id, 1);
        assert_eq!(lockup.withdraw(0), err!(ErrorCode::WithdrawalIsZero));
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_period_after_multiple_stakes() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 1 };

        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 2 };
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 0);
        assert_eq!(lockup.locked_tokens_prev, 1000);
        assert_eq!(lockup.locked_tokens, 1000);
        assert_eq!(lockup.locked_from_period_id, 2);
        assert_eq!(
            lockup.withdraw(1),
            err!(ErrorCode::NoAvailableTokensForWithdrawal)
        );
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_2_periods_after_multiple_stakes() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 1 };

        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 2 };
        lockup.stake_update(1000, status).unwrap();

        let status = SlaStatus::Active { period_id: 3 };
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 1000);
        assert_eq!(lockup.locked_tokens_prev, 1000);
        assert_eq!(lockup.locked_tokens, 1000);
        assert_eq!(lockup.locked_from_period_id, 3);
        assert_eq!(lockup.withdraw(999), Ok(()));
        assert_eq!(lockup.available_tokens, 1);
    }

    #[test]
    fn check_multiple_periods_after_multiple_stakes() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 1 };

        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 2 };
        lockup.stake_update(1000, status).unwrap();

        let status = SlaStatus::Active { period_id: 100 };
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 2000);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 1000);
        assert_eq!(lockup.locked_from_period_id, 100);
        assert_eq!(
            lockup.withdraw(20000),
            err!(ErrorCode::NoAvailableTokensForWithdrawal)
        );
        assert_eq!(lockup.available_tokens, 2000);
    }

    #[test]
    fn check_current_period_sla_not_started() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::NotStarted;
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 0);
        assert_eq!(lockup.locked_tokens_prev, 1000);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 0);
        assert_eq!(
            lockup.withdraw(20000),
            err!(ErrorCode::NoAvailableTokensForWithdrawal)
        );
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_period_after_sla_not_started() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::NotStarted;

        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 0 };
        lockup.update_available_tokens(status).unwrap();

        assert_eq!(lockup.available_tokens, 0);
        assert_eq!(lockup.locked_tokens_prev, 1000);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 0);
        assert_eq!(
            lockup.withdraw(50),
            err!(ErrorCode::NoAvailableTokensForWithdrawal)
        );
        assert_eq!(lockup.available_tokens, 0);
    }
    #[test]
    fn check_2_periods_after_sla_not_started() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::NotStarted;

        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 1 };
        lockup.update_available_tokens(status).unwrap();

        assert_eq!(lockup.available_tokens, 1000);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 1);
        assert_eq!(lockup.withdraw(1000), Ok(()));
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_current_period_multiple_stakes_sla_not_started() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::NotStarted;

        lockup.stake_update(1000, status).unwrap();
        lockup.stake_update(1000, status).unwrap();
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 0);
        assert_eq!(lockup.locked_tokens_prev, 3000);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 0);
        assert_eq!(
            lockup.withdraw(1),
            err!(ErrorCode::NoAvailableTokensForWithdrawal)
        );
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_period_after_multiple_stakes_sla_not_started() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::NotStarted;

        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 0 };
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 0);
        assert_eq!(lockup.locked_tokens_prev, 1000);
        assert_eq!(lockup.locked_tokens, 1000);
        assert_eq!(lockup.locked_from_period_id, 0);
        assert_eq!(
            lockup.withdraw(5000),
            err!(ErrorCode::NoAvailableTokensForWithdrawal)
        );
        assert_eq!(lockup.available_tokens, 0);
    }

    #[test]
    fn check_2_periods_after_multiple_stakes_sla_not_started() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::NotStarted;
        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 0 };
        lockup.stake_update(1000, status).unwrap();

        let status = SlaStatus::Active { period_id: 1 };
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 1000);
        assert_eq!(lockup.locked_tokens_prev, 1000);
        assert_eq!(lockup.locked_tokens, 1000);
        assert_eq!(lockup.locked_from_period_id, 1);
        assert_eq!(lockup.withdraw(1), Ok(()));
        assert_eq!(lockup.available_tokens, 999);
    }

    #[test]
    fn check_multiple_periods_after_multiple_stakes_sla_not_started() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::NotStarted;

        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 1 };
        lockup.stake_update(1000, status).unwrap();

        let status = SlaStatus::Active { period_id: 100 };
        lockup.stake_update(1000, status).unwrap();

        assert_eq!(lockup.available_tokens, 2000);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 1000);
        assert_eq!(lockup.locked_from_period_id, 100);
        assert_eq!(lockup.withdraw(300), Ok(()));
        assert_eq!(lockup.available_tokens, 1700);
    }

    #[test]
    fn check_previous_period_sla_ended() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 100 };
        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Ended;
        lockup.update_available_tokens(status).unwrap();

        assert_eq!(lockup.available_tokens, 1000);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 0);
        assert_eq!(lockup.withdraw(50), Ok(()));
        assert_eq!(lockup.available_tokens, 950);
    }

    #[test]
    fn check_previous_period_multiple_stakes_sla_ended() {
        let mut lockup = Lockup::new();
        let status = SlaStatus::Active { period_id: 50 };
        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 70 };
        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Active { period_id: 100 };
        lockup.stake_update(1000, status).unwrap();
        let status = SlaStatus::Ended;
        lockup.update_available_tokens(status).unwrap();

        assert_eq!(lockup.available_tokens, 3000);
        assert_eq!(lockup.locked_tokens_prev, 0);
        assert_eq!(lockup.locked_tokens, 0);
        assert_eq!(lockup.locked_from_period_id, 0);
        assert_eq!(lockup.withdraw(520), Ok(()));
        assert_eq!(lockup.available_tokens, 2480);
    }
}
