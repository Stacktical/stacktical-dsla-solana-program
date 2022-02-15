use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dsla_stacktical_contracts_solana {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, sla: SLA) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 1024)]
    pub sla: Account<'info, SLA>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub lp_token: Account<'info, TokenAccount>,
    pub sp_token: Account<'info, TokenAccount>,
}

#[account]
pub struct SLA {
    pub slo: SLO,
    pub breached: bool,
    pub timestamp_start: u128,
    pub timestamp_end: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SLO {
    value: i128,
    operand: Operand,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum Operand {
    Greater,
    Lesser,
}

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
