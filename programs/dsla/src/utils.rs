use crate::{
    errors::ErrorCode,
    state::{Slo, SloType},
};
use anchor_lang::prelude::*;
use rust_decimal::Decimal;

/// Calculate deviation between SLO and SLI
/// Ensures a positive deviation for greater / small comparisons
/// The deviation is the percentage difference between SLI and SLO
///                          | sloValue - sli |
/// formula =>  deviation = -------------------- %
///                          (sli + sloValue) / 2
pub fn get_deviation(slo: &Slo, sli: &Decimal) -> Result<Decimal> {
    // 25% as default
    let deviation_cap_rate: Decimal = Decimal::new(25, 2);

    let slo_type = slo.slo_type;
    let slo_value = slo.slo_value.to_decimal();

    let mut deviation: Decimal = sli
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

    match slo_type {
        // Deviation of 1%
        SloType::EqualTo | SloType::NotEqualTo => Ok(deviation_cap_rate),
        _ => Ok(deviation),
    }
}
