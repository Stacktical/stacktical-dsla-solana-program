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
    pub leverage: u64,
    pub ipfs_hash: String,
    /// address of the coin to be used as SLA reward for users and providers
    pub mint_address: Pubkey,
    /// The account derived by the program, which has authority over all
    /// assets in the SLA.
    pub sla_authority: Pubkey,
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
    pub slo_value: Decimal,
    pub slo_type: SloType,
}

impl Slo {
    /// slo_value + slo_type
    pub const LEN: usize = 16 + 1;

    pub fn is_respected(&self, sli: Decimal) -> Result<bool> {
        let slo_type = self.slo_type;
        let slo_value = self.slo_value;

        match slo_type {
            SloType::EqualTo => Ok(sli == slo_value),
            SloType::NotEqualTo => Ok(sli != slo_value),
            SloType::SmallerThan => Ok(sli < slo_value),
            SloType::SmallerOrEqualTo => Ok(sli <= slo_value),
            SloType::GreaterThan => Ok(sli > slo_value),
            SloType::GreaterOrEqualTo => Ok(sli >= slo_value),
        }
    }

    pub fn get_deviation(&self, sli: Decimal, precision: u128) -> Result<Decimal> {
        if (precision % 100 != 0) || (precision == 0) {
            return err!(ErrorCode::InvalidPrecision);
        }
        let precision = Decimal::from_u128(precision).unwrap(); // FIXME: remove unwrap
        let slo_type = self.slo_type;
        let slo_value = self.slo_value;

        let mut deviation: Decimal = (if sli >= slo_value {
            sli - slo_value
        } else {
            slo_value
        }) * precision
            / (sli + slo_value)
            / (Decimal::new(2, 0));

        if deviation > (precision * (Decimal::new(25, 0)) / (Decimal::new(100, 0))) {
            deviation = precision * (Decimal::new(25, 0)) / (Decimal::new(100, 0));
        }
        match slo_type {
            // Deviation of 1%
            SloType::EqualTo | SloType::NotEqualTo => Ok(precision / (Decimal::new(100, 0))),
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
