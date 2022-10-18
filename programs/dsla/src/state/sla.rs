use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use rust_decimal::{prelude::FromPrimitive, Decimal};

#[account]
pub struct SlaAuthority {}
#[account]
pub struct Sla {
    /// address of the messeger providing the data
    pub messenger_address: Pubkey,
    /// service level objective, the objective to achieve for the provider to be rewarded
    pub slo: Slo,
    ///  leverage for the SLA between provider and user pool
    pub leverage: DslaDecimal,
    pub ipfs_hash: String,
    /// address of the coin to be used as SLA reward for users and providers
    pub mint_address: Pubkey,
    /// The account derived by the program, which has authority over all
    /// assets in the SLA.
    pub sla_authority: Pubkey,
    /// all the data regarding periods.
    pub period_data: PeriodGenerator,
    /// all the reward availaible for future stakes.
    pub total_liquidity_available: u128,
    /// The address used as the seed for generating the SLA authority
    /// address. Typically this is the SLA account's own address.
    pub authority_seed: Pubkey,
    /// The bump seed value for generating the authority address.
    pub authority_bump_seed: [u8; 1],
}

impl Sla {
    // discriminator + messenger_address + SLO + leverage + ipfs_hash + mint + authority + mint_address
    pub const LEN: usize = 8 + 32 + Slo::LEN + 8 + 32 + 32 + 32 + 16 + 16 + 32;
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Slo {
    pub slo_value: DslaDecimal,
    pub slo_type: SloType,
}

impl Slo {
    /// slo_value + slo_type
    pub const LEN: usize = 64 + 1; // FIXME: found out and fix for size of Decimal

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

    pub fn get_deviation(&self, sli: DslaDecimal, precision: u128) -> Result<Decimal> {
        if (precision % 100 != 0) || (precision == 0) {
            return err!(ErrorCode::InvalidPrecision);
        }
        let sli = sli.to_decimal();

        let precision = Decimal::from_u128(precision).ok_or(ErrorCode::DecimalConversionError)?;
        let slo_type = self.slo_type;
        let slo_value = self.slo_value.to_decimal();

        let mut deviation: Decimal = (if sli >= slo_value {
            sli.checked_sub(slo_value)
                .ok_or(ErrorCode::CheckedOperationOverflow)?
        } else {
            slo_value
        })
        .checked_mul(precision)
        .ok_or(ErrorCode::CheckedOperationOverflow)?
        .checked_div(
            sli.checked_add(slo_value)
                .ok_or(ErrorCode::CheckedOperationOverflow)?,
        )
        .ok_or(ErrorCode::CheckedOperationOverflow)?
        .checked_div(Decimal::new(2, 0))
        .ok_or(ErrorCode::CheckedOperationOverflow)?;

        if deviation
            > (precision
                .checked_mul(Decimal::new(25, 0))
                .ok_or(ErrorCode::CheckedOperationOverflow)?
                .checked_div(Decimal::new(100, 0)))
            .ok_or(ErrorCode::CheckedOperationOverflow)?
        {
            deviation = precision
                .checked_mul(Decimal::new(25, 0))
                .ok_or(ErrorCode::CheckedOperationOverflow)?
                .checked_div(Decimal::new(100, 0))
                .ok_or(ErrorCode::CheckedOperationOverflow)?;
        }
        match slo_type {
            // Deviation of 1%
            SloType::EqualTo | SloType::NotEqualTo => Ok(precision
                .checked_div(Decimal::new(100, 0))
                .ok_or(ErrorCode::CheckedOperationOverflow)?),
            _ => Ok(deviation),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy)]
pub enum SloType {
    EqualTo,
    NotEqualTo,
    SmallerThan,
    SmallerOrEqualTo,
    GreaterThan,
    GreaterOrEqualTo,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct DslaDecimal {
    mantissa: i64,
    scale: u32,
}

impl DslaDecimal {
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

/// struct used to generate the periods for an SLA
///
///
/// # Fields
///
///  * `start` - the start of the first period in unix time (must be positive we are in 2022)
///  * `period_length` - the length of each period
///  * `n_periods` - number of periods
#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Clone)]
pub struct PeriodGenerator {
    pub start: u128,
    pub period_length: PeriodLength,
    pub n_periods: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, Clone)]
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

    pub fn new(start: u128, period_length: PeriodLength, n_periods: u128) -> Self {
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
    pub fn get_start(&self, period_id: u128) -> Result<u128> {
        require_gt!(self.n_periods, period_id, ErrorCode::InvalidPeriodId);
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
        // TODO: to be tested using the client needs the underlying blockchain for time
        let timestamp = Clock::get()?.unix_timestamp as u128;
        Ok(timestamp >= self.get_start(period_id)?)
    }
    pub fn has_finished(&self, period_id: u128) -> Result<bool> {
        // TODO: to be tested using the client needs the underlying blockchain for time
        let timestamp = Clock::get()?.unix_timestamp as u128;
        Ok(timestamp > self.get_end(period_id)?)
    }

    pub fn get_current_period_id(&self) -> Result<u128> {
        // TODO: to be tested using the client needs the underlying blockchain for time
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        // require SLA is currently active
        require_gte!(self.start, current_timestamp, ErrorCode::SlaNotStarted);
        require_gt!(
            current_timestamp,
            self.get_end(self.n_periods - 1)?,
            ErrorCode::SlaAlreadyEnded
        );

        match self.period_length {
            PeriodLength::Custom {
                length: period_length,
            } => Ok((current_timestamp - self.start) / period_length),
            PeriodLength::Monthly => unimplemented!(),
            PeriodLength::Yearly => unimplemented!(),
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
