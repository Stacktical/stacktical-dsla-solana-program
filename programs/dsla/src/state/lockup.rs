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

    pub fn update_available_tokens(&mut self, status: SlaStatus) {
        match status {
            SlaStatus::NotStarted => {
                self.locked_tokens = 0;
                self.locked_tokens_prev = 0;
                self.available_tokens = 0;
            }
            SlaStatus::Active { period_id } => {
                let period_id = period_id as u64;

                match period_id.cmp(&(self.locked_from_period_id + 1)) {
                    Ordering::Greater => {
                        self.available_tokens += self.locked_tokens_prev + self.locked_tokens;
                        self.locked_tokens_prev = 0;
                        self.locked_tokens = 0;
                        self.locked_from_period_id = period_id;
                    }
                    Ordering::Less => {}
                    Ordering::Equal => {
                        self.available_tokens += self.locked_tokens_prev;
                        self.locked_tokens_prev = self.locked_tokens;
                        self.locked_tokens = 0;
                        self.locked_from_period_id = period_id;
                    }
                }
            }
            SlaStatus::Ended => {
                self.available_tokens += self.locked_tokens + self.locked_tokens_prev;
                self.locked_tokens = 0;
                self.locked_tokens_prev = 0;
                self.locked_from_period_id = 0;
            }
        }
    }

    pub fn stake_update(&mut self, stake_size: u64, status: SlaStatus) {
        match status {
            SlaStatus::NotStarted => {
                self.locked_tokens += stake_size;
                self.locked_tokens = 0;
                self.locked_tokens_prev = 0;
                self.available_tokens = 0;
            }
            SlaStatus::Active { period_id } => {
                let period_id = period_id as u64;
                if self.locked_from_period_id == period_id {
                    self.locked_tokens += stake_size;
                }
                match period_id.cmp(&(self.locked_from_period_id + 1)) {
                    Ordering::Greater => {
                        self.available_tokens += self.locked_tokens_prev + self.locked_tokens;
                        self.locked_tokens_prev = 0;
                        self.locked_tokens = stake_size;
                        self.locked_from_period_id = period_id;
                    }
                    Ordering::Less => {}
                    Ordering::Equal => {
                        self.available_tokens += self.locked_tokens_prev;
                        self.locked_tokens_prev = self.locked_tokens;
                        self.locked_tokens = stake_size;
                        self.locked_from_period_id = period_id;
                    }
                }
            }

            SlaStatus::Ended => {}
        }
    }

    pub fn withdraw(&mut self, withdraw_size: u64) {
        self.available_tokens -= withdraw_size;
    }
}
