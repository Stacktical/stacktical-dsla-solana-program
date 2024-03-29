use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use rust_decimal::Decimal;

/// `Sla` is Service level agreement account containing all the variables to make it possible
#[account]
pub struct Sla {
    /// address of who deployed the SLA
    pub sla_deployer_address: Pubkey,
    /// address of the switchboard aggregator account
    pub aggregator_address: Pubkey,
    /// service level objective, the objective to achieve for the provider to be rewarded
    pub slo: Slo,
    ///  leverage for the SLA between provider and user pool
    pub leverage: DslaDecimal,
    /// address of the coin to be used as SLA reward for users and providers
    pub mint_address: Pubkey,
    /// all the data regarding periods.
    pub period_data: PeriodGenerator,
    /// amount of tokens in Provider pool
    pub provider_pool_size: u128,
    /// amount of tokens in User pool
    pub user_pool_size: u128,
    /// total user token supply
    pub ut_supply: u128,
    /// total provider token supply
    pub pt_supply: u128,
    /// range of severity max of 10;
    pub severity: Vec<DslaDecimal>,
    /// range of penalty max of 10;
    pub penalty: Vec<DslaDecimal>,
}

impl Sla {
    pub const LEN: usize = 8 + // discriminator
        32 + // aggregator_address
        32 + // sla_deployer_address
        32 + // messenger_address
        Slo::LEN + // SLO
        12 + // leverage
        32 + // mint_address
        PeriodGenerator::LEN + // period_data
        16 + // provider_pool_size
        16 + // user_pool_size
        16 + // ut_supply
        16 + // pt_supply
        4 + (DslaDecimal::LEN * 10) + // severity
        4 + (DslaDecimal::LEN * 10); // penalty

    /// Calculate deviation between SLO and SLI
    /// Ensures a positive deviation for greater / small comparisons
    /// The default deviation is the percentage difference between SLI and SLO
    ///                          | sloValue - sli |
    /// formula =>  deviation = -------------------- %
    ///                          (sli + sloValue) / 2
    /// if the penalty is set to 0, default deviation will be used
    /// if the panlty is not set, default deviation will be used
    pub fn get_deviation(&self, sli: &Decimal) -> Result<Decimal> {
        let mut deviation = Decimal::new(0, 0);

        for (i, s) in self.severity.iter().enumerate() {
            if sli.ge(&s.to_decimal()) {
                deviation = self.penalty[i].to_decimal();
            }
        }

        let slo_type = self.slo.slo_type;
        // 25% as default
        let deviation_cap_rate: Decimal = Decimal::new(25, 2);

        if deviation.eq(&Decimal::new(0, 0)) {
            let slo_value = self.slo.slo_value.to_decimal();

            deviation = sli
                .checked_sub(slo_value)
                .ok_or(ErrorCode::CheckedOperationOverflow)?
                .abs()
                .checked_div(
                    sli.checked_add(slo_value)
                        .ok_or(ErrorCode::CheckedOperationOverflow)?
                        .checked_div(Decimal::new(2, 0))
                        .ok_or(ErrorCode::CheckedOperationOverflow)?,
                )
                .ok_or(ErrorCode::CheckedOperationOverflow)?;

            if deviation > (deviation_cap_rate) {
                deviation = deviation_cap_rate;
            }
        }

        match slo_type {
            // Deviation of 1%
            SloType::EqualTo | SloType::NotEqualTo => Ok(deviation_cap_rate),
            _ => Ok(deviation),
        }
    }
}

/// `Slo` is service level objective and contains a Decimal number that is the expected value and  SloType
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Slo {
    pub slo_value: DslaDecimal,
    pub slo_type: SloType,
}

impl Slo {
    /// slo_value + slo_type
    pub const LEN: usize = DslaDecimal::LEN + SloType::LEN; // @remind find out and fix for size of Decimal

    pub fn is_respected(&self, sli: DslaDecimal) -> Result<bool> {
        let slo_type = self.slo_type;
        let slo_value = self.slo_value.to_decimal();
        let sli = sli.to_decimal();

        match slo_type {
            SloType::EqualTo => Ok(sli == slo_value),
            SloType::NotEqualTo => Ok(sli != slo_value),
            SloType::SmallerThan => Ok(sli < slo_value),
            SloType::SmallerOrEqualTo => Ok(sli <= slo_value),
            SloType::GreaterThan => Ok(sli > slo_value),
            SloType::GreaterOrEqualTo => Ok(sli >= slo_value),
        }
    }
}

/// what type of service level objective is this `Slo`
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy)]
pub enum SloType {
    EqualTo,
    NotEqualTo,
    SmallerThan,
    SmallerOrEqualTo,
    GreaterThan,
    GreaterOrEqualTo,
}

impl SloType {
    pub const LEN: usize = 1 + 1;
}

/// struct to deal with floating point numbers
#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct DslaDecimal {
    /// the value without any decimals and non decimal
    mantissa: i64,
    /// how many places from the right to put the decimal point
    scale: u32,
}

impl DslaDecimal {
    pub const LEN: usize = 8 + 4;
    pub fn to_decimal(&self) -> Decimal {
        Decimal::new(self.mantissa, self.scale)
    }

    pub fn from_decimal(decimal: Decimal) -> Self {
        Self {
            mantissa: decimal.mantissa() as i64,
            scale: decimal.scale(),
        }
    }
}

/// struct used to generate the periods for an SLA with helper function to retrieve any period
#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Clone)]
pub struct PeriodGenerator {
    /// the first timestamp indicating the beginning of the SLA and of the first period
    pub start: u128,
    /// the length of each period
    pub period_length: PeriodLength,
    /// number of periods
    pub n_periods: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Clone)]
pub enum PeriodLength {
    Custom { length: u128 },
    Monthly,
    Yearly,
}

impl PeriodLength {
    const LEN: usize = 1 + 16;
}

/// The `SlaStatus` is in an enum to define the status of the `Sla`
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum SlaStatus {
    NotStarted,
    Active { period_id: u32 },
    Ended,
}

impl PeriodGenerator {
    /// start + period_length + n_periods
    pub const LEN: usize = 16 + PeriodLength::LEN + 4;

    /// minumum delay from now for the creation of a new period generator
    pub const MIN_DELAY: u128 = 600000;
    /// minumum delay from now for the creation of a new period generator
    pub const MIN_PERIOD_LENGTH: u128 = 60000;

    /// return a new period generator object
    pub fn new(start: u128, period_length: PeriodLength, n_periods: u32) -> Self {
        Self {
            start,
            period_length,
            n_periods,
        }
    }

    /// Returns the start timestamp of a given period id
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to get the start timestamp of
    pub fn get_start(&self, period_id: usize) -> Result<u128> {
        require_gt!(
            self.n_periods as usize,
            period_id,
            ErrorCode::InvalidPeriodId
        );
        match self.period_length {
            PeriodLength::Custom { length } => Ok(self
                .start
                .checked_add(length.checked_mul(period_id as u128).unwrap())
                .unwrap()),
            PeriodLength::Monthly => unimplemented!(),
            PeriodLength::Yearly => unimplemented!(),
        }
    }
    /// Returns the end timestamp of a given period id
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to get the end timestamp of
    pub fn get_end(&self, period_id: usize) -> Result<u128> {
        match self.period_length {
            PeriodLength::Custom { length } => {
                msg!(length.to_string().as_ref());
                Ok(self
                    .get_start(period_id as usize)?
                    .checked_add(length.checked_sub(1).unwrap())
                    .unwrap())
            }
            PeriodLength::Monthly => unimplemented!(),
            PeriodLength::Yearly => unimplemented!(),
        }
    }

    /// Returns the if a given period id has started
    ///
    /// # Arguments
    ///
    /// * `period_id` - the period id of which to check if it has started
    pub fn has_started(&self, period_id: usize) -> Result<bool> {
        // @remind to be tested using the client needs the underlying blockchain for time
        let timestamp = Clock::get()?.unix_timestamp as u128;
        Ok(timestamp >= self.get_start(period_id)?)
    }
    pub fn has_finished(&self, period_id: usize) -> Result<bool> {
        // @remind to be tested using the client needs the underlying blockchain for time
        let timestamp = Clock::get()?.unix_timestamp as u128;
        Ok(timestamp > self.get_end(period_id)?)
    }

    /// returns an enum `SlaStatus` with the current period id if the sla is active
    pub fn get_current_period_id(&self) -> Result<SlaStatus> {
        // @remind to be tested using the client needs the underlying blockchain for time
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        if current_timestamp > self.get_end((self.n_periods.checked_sub(1).unwrap()) as usize)? {
            Ok(SlaStatus::Ended)
        } else if self.start >= current_timestamp {
            Ok(SlaStatus::NotStarted)
        } else {
            match self.period_length {
                PeriodLength::Custom { length } => {
                    // @remind look into this division might cause problems
                    let period_id = ((current_timestamp.checked_sub(self.start).unwrap())
                        .checked_div(length)
                        .unwrap()) as usize;
                    Ok(SlaStatus::Active {
                        period_id: period_id as u32,
                    })
                }
                PeriodLength::Monthly => unimplemented!(),
                PeriodLength::Yearly => unimplemented!(),
            }
        }
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
        };
        period.get_start(10).unwrap();
    }
    #[test]
    fn get_start_valid_last() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 10,
        };
        assert_eq!(period.get_start(9).unwrap(), 550);
    }
    #[test]
    fn get_end_valid_id_1() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 10,
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
        };
        period.get_end(10).unwrap();
    }
    #[test]
    fn get_end_valid_last() {
        let period = PeriodGenerator {
            start: 100,
            period_length: PeriodLength::Custom { length: 50 },
            n_periods: 10,
        };
        assert_eq!(period.get_end(9).unwrap(), 599);
    }
}
