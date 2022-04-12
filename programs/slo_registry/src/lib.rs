use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod slo_registry {
    use super::*;

    pub fn register_slo(
        ctx: Context<CreateSlo>,
        sla_address: Pubkey,
        slo_type: SloType,
        slo_value: u128,
    ) -> Result<()> {
        let slo = &mut ctx.accounts.slo;
        slo.slo_type = slo_type;
        slo.slo_value = slo_value;
        slo.bump = *match ctx.bumps.get("slo") {
            Some(bump) => bump,
            None => return err!(ErrorCode::SloNotFound),
        };
        emit!(RegisteredSlo {
            sla_address,
            slo_type,
            slo_value
        });
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(sla_address: Pubkey)]
pub struct CreateSlo<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // space: 8 discriminator + Slo max size
    #[account(
        init,
        payer = user,
        space = 8 + Slo::MAX_SIZE,
        seeds = [b"slo", user.key().as_ref(), sla_address.as_ref()],
        bump
    )]
    pub slo: Account<'info, Slo>,
    pub system_program: Program<'info, System>,
}

impl Slo {
    pub fn is_respected(&self, value: u128) -> Result<bool> {
        let slo_type = self.slo_type;
        let slo_value = self.slo_value;

        match slo_type {
            SloType::EqualTo => Ok(value == slo_value),
            SloType::NotEqualTo => Ok(value != slo_value),
            SloType::SmallerThan => Ok(value < slo_value),
            SloType::SmallerOrEqualTo => Ok(value <= slo_value),
            SloType::GreaterThan => Ok(value > slo_value),
            SloType::GreaterOrEqualTo => Ok(value >= slo_value),
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
            SloType::EqualTo | SloType::NotEqualTo => Ok(precision / 100),
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

#[account]
pub struct Slo {
    pub slo_value: u128,
    pub slo_type: SloType,
    bump: u8,
}

impl Slo {
    // slo_value + slo_type + bump
    pub const MAX_SIZE: usize = 16 + 1 + 1;
}

#[error_code]
pub enum ErrorCode {
    #[msg("the SLA address provided does not have a Slo registered.")]
    SloNotFound,
    #[msg("precision is not divisible by 100")]
    InvalidPrecision,
}

#[event]
pub struct RegisteredSlo {
    pub sla_address: Pubkey,
    pub slo_value: u128,
    pub slo_type: SloType,
}
