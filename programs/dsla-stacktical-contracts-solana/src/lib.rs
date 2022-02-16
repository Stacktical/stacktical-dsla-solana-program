use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dsla_stacktical_contracts_solana {
    use super::*;
    pub fn initialize_sla(
        ctx: Context<Initialize>,
        slo_value: i128,
        slo_operand: Operand,
        timestamp_start: u128,
        duration: u128,
    ) -> ProgramResult {
        let sla = &mut ctx.accounts.sla;
        sla.slo_value = slo_value;
        sla.slo_operand = slo_operand;
        sla.breached = false;
        sla.timestamp_start = timestamp_start;
        sla.duration = duration;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64, position: Position) -> ProgramResult {
        // get token amount and store to
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.lp_token.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.dsla_pool.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
                &[&[&"stake".as_bytes()]],
            ),
            amount,
        )?;

        match position {
            Position::Long => {
                anchor_spl::token::mint_to(
                    CpiContext::new_with_signer(
                        ctx.accounts.lp_token.to_account_info(),
                        anchor_spl::token::MintTo {
                            mint: ctx.accounts.mint.to_account_info(),
                            to: ctx.accounts.payer.to_account_info(),
                            authority: ctx.accounts.mint.to_account_info(),
                        },
                        &[&[&"mint".as_bytes()]],
                    ),
                    amount,
                )?;
            }

            Position::Short => {
                anchor_spl::token::mint_to(
                    CpiContext::new_with_signer(
                        ctx.accounts.sp_token.to_account_info(),
                        anchor_spl::token::MintTo {
                            mint: ctx.accounts.mint.to_account_info(),
                            to: ctx.accounts.payer.to_account_info(),
                            authority: ctx.accounts.mint.to_account_info(),
                        },
                        &[&[&"mint".as_bytes()]],
                    ),
                    amount,
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 1024)]
    pub sla: Account<'info, SLA>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(mint_bump: u8, amount: u64)]
pub struct Stake<'info> {
    pub payer: Signer<'info>,
    pub lp_token: Account<'info, TokenAccount>,
    pub sp_token: Account<'info, TokenAccount>,
    pub dsla_pool: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = payer,
        seeds = [b"mint".as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct SLA {
    pub slo_value: i128,
    pub slo_operand: Operand,
    pub breached: bool,
    pub timestamp_start: u128,
    pub duration: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum Operand {
    Greater,
    Lesser,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum Position {
    Long,
    Short,
}

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
