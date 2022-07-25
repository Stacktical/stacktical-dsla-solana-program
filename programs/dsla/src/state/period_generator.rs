use anchor_lang::prelude::*;

use crate::errors::ErrorCode;

/// struct used to generate the periods for an SLA
///
///
/// # Fields
///
///  * `start` - the start of the first period in unix time (must be positive we are in 2022)
///  * `period_length` - the length of each period
///  * `n_periods` - number of periods
#[account]
pub struct PeriodGenerator {
    pub start: u128,
    pub period_length: PeriodLength,
    pub n_periods: u128,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Clone)]
pub enum PeriodLength {
    Custom { length: u128 },
    Monthly,
    Yearly,
}

impl PeriodGenerator {
    /// start + period_length + n_periods + bump
    pub const MAX_SIZE: usize = 16 + 16 + 16 + 1;

    /// minumum delay from now for the creation of a new period generator
    pub const MIN_DELAY: u128 = 600000;
    /// minumum delay from now for the creation of a new period generator
    pub const MIN_PERIOD_LENGTH: u128 = 60000;

    /// Returns the start timestamp of a given period id
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to get the start timestamp of
    pub fn get_start(&self, period_id: u128) -> Result<u128> {
        require!(period_id < self.n_periods, ErrorCode::InvalidPeriodId);

        match self.period_length {
            PeriodLength::Custom {
                length: period_length,
            } => Ok(self.start + (period_length * period_id)),
            PeriodLength::Monthly => unimplemented!(),
            PeriodLength::Yearly => unimplemented!(),
        }
    }
    /// Returns the end timestamp of a given period id
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to get the end timestamp of
    pub fn get_end(&self, period_id: u128) -> Result<u128> {
        match self.period_length {
            PeriodLength::Custom {
                length: period_length,
            } => Ok(self.get_start(period_id)? + (period_length - 1)),
            PeriodLength::Monthly => unimplemented!(),
            PeriodLength::Yearly => unimplemented!(),
        }
    }

    /// Returns the if a given period id has started
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to check if it has started
    pub fn has_started(&self, period_id: u128) -> Result<bool> {
        // TODO: to be test using the client needs the underlying blockchain for time
        let timestamp = Clock::get()?.unix_timestamp as u128;
        Ok(timestamp >= self.get_start(period_id)?)
    }
    pub fn has_finished(&self, period_id: u128) -> Result<bool> {
        // TODO: to be test using the client needs the underlying blockchain for time
        let timestamp = Clock::get()?.unix_timestamp as u128;
        Ok(timestamp > self.get_end(period_id)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_start_valid_id_1() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 2,
            bump: 1,
        };
        assert_eq!(period.get_start(1).unwrap(), 150);
    }
    #[test]
    #[should_panic]
    fn get_start_id_too_large() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 10,
            bump: 1,
        };
        period.get_start(10).unwrap();
    }
    #[test]
    fn get_start_valid_last() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 10,
            bump: 1,
        };
        assert_eq!(period.get_start(9).unwrap(), 550);
    }
    #[test]
    fn get_end_valid_id_1() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 10,
            bump: 1,
        };
        assert_eq!(period.get_end(1).unwrap(), 199);
    }
    #[test]
    #[should_panic]
    fn get_end_id_too_large() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 10,
            bump: 1,
        };
        period.get_end(10).unwrap();
    }
    #[test]
    fn get_end_valid_last() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 10,
            bump: 1,
        };
        assert_eq!(period.get_end(9).unwrap(), 599);
    }
}
