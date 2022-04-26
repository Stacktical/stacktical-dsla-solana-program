use anchor_lang::prelude::*;

use crate::errors::ErrorCode;

/// struct used to generate the periods for an SLA
///
///
/// # Fields
///
///  * `periods` - the timestamps of each step
///
#[account]
pub struct PeriodRegistry {
    pub periods: Vec<Period>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Period {
    pub start: u64,
    pub end: u64,
    pub status: Status,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum Status {
    NotVerified,
    Respected { value: u128 },
    NotRespected { value: u128 },
}

impl Period {
    pub const MAX_SIZE: usize = 8 + 8 + 1 + 16;
}

impl PeriodRegistry {
    /// minumum delay from now to the first period start
    pub const MIN_DELAY: u64 = 600000;
    /// minumum delay beetween period
    pub const MIN_PERIOD_LENGTH: u64 = 60000;

    pub fn verify_period_length(periods: &[Period]) -> bool {
        for (i, period) in periods.iter().enumerate() {
            dbg!(period);

            if period.end < (period.start + PeriodRegistry::MIN_PERIOD_LENGTH) {
                return false;
            }

            if i < (periods.len() - 1) {
                if period.end > periods[i + 1].start {
                    return false;
                };
            } else {
                return true;
            }
        }
        true
    }

    /// Returns the start timestamp of a given period id
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to get the start timestamp of
    pub fn get_start(&self, period_id: usize) -> Result<u64> {
        require!(period_id < self.periods.len(), ErrorCode::InvalidPeriodId);
        Ok(self.periods[period_id].start)
    }
    /// Returns the end timestamp of a given period id
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to get the end timestamp of
    pub fn get_end(&self, period_id: usize) -> Result<u64> {
        require!(period_id < (self.periods.len()), ErrorCode::InvalidPeriodId);
        Ok(self.periods[period_id].end)
    }
    /// Returns the if a given period id has started
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to check if it has started
    pub fn has_started(&self, period_id: usize) -> Result<bool> {
        // TODO: to be test using the client needs the underlying blockchain for time
        let timestamp = Clock::get()?.unix_timestamp as u64;
        Ok(timestamp >= self.get_start(period_id)?)
    }
    pub fn has_finished(&self, period_id: usize) -> Result<bool> {
        // TODO: to be test using the client needs the underlying blockchain for time
        let timestamp = Clock::get()?.unix_timestamp as u64;
        Ok(timestamp > self.get_end(period_id)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_period_registry_1() -> PeriodRegistry {
        PeriodRegistry {
            periods: vec![
                Period {
                    start: 100,
                    end: 200,
                    status: Status::Respected { value: 100 },
                },
                Period {
                    start: 200,
                    end: 300,
                    status: Status::Respected { value: 99 },
                },
                Period {
                    start: 300,
                    end: 400,
                    status: Status::NotRespected { value: 50 },
                },
                Period {
                    start: 400,
                    end: 500,
                    status: Status::NotVerified,
                },
                Period {
                    start: 500,
                    end: 600,
                    status: Status::NotVerified,
                },
                Period {
                    start: 600,
                    end: 700,
                    status: Status::NotVerified,
                },
                Period {
                    start: 700,
                    end: 800,
                    status: Status::NotVerified,
                },
                Period {
                    start: 800,
                    end: 900,
                    status: Status::NotVerified,
                },
                Period {
                    start: 900,
                    end: 1000,
                    status: Status::NotVerified,
                },
                Period {
                    start: 1000,
                    end: 1100,
                    status: Status::NotVerified,
                },
            ],
            bump: 1,
        }
    }

    fn get_period_vec_1() -> Vec<Period> {
        vec![
            Period {
                start: 60000,
                end: 120000,
                status: Status::NotVerified,
            },
            Period {
                start: 120000,
                end: 180000,
                status: Status::NotVerified,
            },
            Period {
                start: 180000,
                end: 240000,
                status: Status::NotVerified,
            },
        ]
    }

    fn get_period_vec_2() -> Vec<Period> {
        vec![
            Period {
                start: 60000,
                end: 119999,
                status: Status::NotVerified,
            },
            Period {
                start: 119999,
                end: 180000,
                status: Status::NotVerified,
            },
            Period {
                start: 180000,
                end: 240000,
                status: Status::NotVerified,
            },
        ]
    }

    fn get_period_vec_3() -> Vec<Period> {
        vec![
            Period {
                start: 60000,
                end: 120000,
                status: Status::NotVerified,
            },
            Period {
                start: 119999,
                end: 180000,
                status: Status::NotVerified,
            },
            Period {
                start: 180000,
                end: 240000,
                status: Status::NotVerified,
            },
        ]
    }
    #[test]
    fn get_start_valid_id_1() {
        let period_registry = get_period_registry_1();
        assert_eq!(period_registry.get_start(1).unwrap(), 200);
    }
    #[test]
    #[should_panic]
    fn get_start_id_too_large() {
        let period_registry = get_period_registry_1();
        period_registry.get_start(10).unwrap();
    }
    #[test]
    fn get_start_valid_last() {
        let period_registry = get_period_registry_1();
        assert_eq!(period_registry.get_start(8).unwrap(), 900);
    }
    #[test]
    fn get_end_valid_id_1() {
        let period_registry = get_period_registry_1();
        assert_eq!(period_registry.get_end(1).unwrap(), 300);
    }
    #[test]
    #[should_panic]
    fn get_end_id_too_large() {
        let period_registry = get_period_registry_1();
        period_registry.get_end(10).unwrap();
    }
    #[test]
    fn get_end_valid_last() {
        let period_registry = get_period_registry_1();
        assert_eq!(period_registry.get_end(8).unwrap(), 1000);
    }
    #[test]
    fn verify_period_length_valid() {
        let vector = get_period_vec_1();
        assert!(PeriodRegistry::verify_period_length(&vector));
    }
    #[test]
    fn verify_period_length_invalid_period_length() {
        let vector = get_period_vec_2();
        assert!(!PeriodRegistry::verify_period_length(&vector));
    }
    #[test]
    fn verify_period_length_invalid_periods() {
        let vector = get_period_vec_3();
        assert!(!PeriodRegistry::verify_period_length(&vector));
    }
}
