use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stacktical_dsla_contracts_solana {
    use super::*;

    const SLA_PDA_SEED: &[u8] = b"sla";

    pub fn initialize(
        ctx: Context<Initialize>,
        slo_value: i128,
        slo_operand: Operand,
        timestamp_start: u128,
        duration: u128,
        initializer_amount: u64,
    ) -> Result<()> {
        let sla = &mut ctx.accounts.sla;
        sla.slo_value = slo_value;
        sla.slo_operand = slo_operand;
        sla.breached = false;
        sla.timestamp_start = timestamp_start;
        sla.duration = duration;

        let (vault_authority, _) = Pubkey::find_program_address(&[SLA_PDA_SEED], ctx.program_id);
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            initializer_amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(initializer_amount: u64)]
pub struct Initialize<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"token-seed".as_ref()],
        bump,
        payer = initializer,
        token::mint = mint,
        token::authority = initializer,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(zero)]
    pub sla: Box<Account<'info, SLA>>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct SLA {
    pub slo_value: i128,
    pub slo_operand: Operand,
    pub breached: bool,
    pub timestamp_start: u128,
    pub duration: u128,
}

#[account]
pub struct Metadata {
    pub sla_address: Pubkey,
    pub uri: String,
    pub symbol: String,
    pub name: String,
    pub position: Position,
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

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}

impl<'info> Initialize<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .initializer_deposit_token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}
