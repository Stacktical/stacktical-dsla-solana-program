use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT");
/// anybody can create an SLO
#[program]
pub mod slo_registry {
    use super::*;

    pub fn register_slo(
        ctx: Context<CreateSLO>,
        _sla_address: Pubkey,
        slo_type: SLOType,
        slo_value: u128,
    ) -> Result<()> {
        let slo = &mut ctx.accounts.slo;

        slo.slo_type = slo_type;
        slo.slo_value = slo_value;
        slo.bump = *match ctx.bumps.get("slo") {
            Some(bump) => bump,
            None => return err!(ErrorCode::SLONotFound),
        };
        emit!(RegisteredSLO {
            sla_address: _sla_address,
            slo_value: slo.slo_value,
            slo_type: slo.slo_type
        });
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_sla_address: Pubkey)]
pub struct CreateSLO<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    // space: 8 discriminator + SLO max size
    #[account(
        init,
        payer = authority,
        space = 8 + SLO::MAX_SIZE,
        seeds = [b"slo", authority.key().as_ref(), _sla_address.as_ref()],
        bump

    )]
    pub slo: Account<'info, SLO>,
    pub system_program: Program<'info, System>,
}

impl SLO {
    pub fn is_respected(&self, value: u128) -> Result<bool> {
        let slo_type = self.slo_type;
        let slo_value = self.slo_value;

        match slo_type {
            SLOType::EqualTo => Ok(value == slo_value),
            SLOType::NotEqualTo => Ok(value != slo_value),
            SLOType::SmallerThan => Ok(value < slo_value),
            SLOType::SmallerOrEqualTo => Ok(value <= slo_value),
            SLOType::GreaterThan => Ok(value > slo_value),
            SLOType::GreaterOrEqualTo => Ok(value >= slo_value),
        }
    }

    pub fn get_deviation(&self, sli: u128, precision: u128) -> Result<u128> {
        if (precision % 100 != 0) || (precision == 0) {
            return err!(ErrorCode::InvalidPrecision);
        }

        let slo_type = self.slo_type;
        let slo_value = self.slo_value;

        let mut deviation: u128 = (if sli >= slo_value {
            sli - slo_value
        } else {
            slo_value
        }) * precision
            / ((sli + slo_value) / 2);

        if deviation > (precision * 25 / 100) {
            deviation = precision * 25 / 100;
        }
        match slo_type {
            // Deviation of 1%
            SLOType::EqualTo | SLOType::NotEqualTo => Ok(precision / 100),
            _ => Ok(deviation),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy)]
pub enum SLOType {
    EqualTo,
    NotEqualTo,
    SmallerThan,
    SmallerOrEqualTo,
    GreaterThan,
    GreaterOrEqualTo,
}

#[account]
pub struct SLO {
    pub slo_value: u128,
    pub slo_type: SLOType,
    bump: u8,
}

impl SLO {
    // slo_value + slo_type + bump
    pub const MAX_SIZE: usize = 16 + 1 + 1;
}

#[error_code]
pub enum ErrorCode {
    #[msg("the SLA address provided does not have a SLO registered.")]
    SLONotFound,
    #[msg("precision is not divisible by 100")]
    InvalidPrecision,
}

#[event]
pub struct RegisteredSLO {
    pub sla_address: Pubkey,
    pub slo_value: u128,
    pub slo_type: SLOType,
}
