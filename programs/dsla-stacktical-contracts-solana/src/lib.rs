use anchor_spl::token::{Mint, Token, TokenAccount};
use {
    anchor_lang::{
        prelude::*, solana_program::program::invoke, AnchorDeserialize, AnchorSerialize, Key,
    },
    metaplex_token_metadata::instruction::create_metadata_accounts,
};
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
        let data: Metadata = Metadata {
            sla_address: ctx.program_id.clone(),
            uri: String::new(),
            name: String::from(""),
            symbol: String::from(""),
            position,
        }; // TODO: sla stake rapresentation
           // get token amount and store to DSLA pool
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.dsla_pool.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.dsla_pool.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
                &[&[&"stake".as_bytes()]],
            ),
            amount,
        )?;

        let creator: metaplex_token_metadata::state::Creator =
            metaplex_token_metadata::state::Creator {
                address: ctx.accounts.payer.key(),
                verified: true,
                share: 0,
            };

        invoke(
            &create_metadata_accounts(
                ctx.accounts.p_nft.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(), // FIXME:change authority to be the program
                data.name,
                data.symbol,
                data.uri,
                Some(vec![creator]),
                0,
                true,
                false,
            ),
            &[
                ctx.accounts.metadata.to_account_info().clone(),
                ctx.accounts.mint.to_account_info().clone(),
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts
                    .token_metadata_program
                    .to_account_info()
                    .clone(),
                ctx.accounts.token_program.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
                ctx.accounts.rent.to_account_info().clone(),
            ],
        )?;

        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.p_nft.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.mint.to_account_info(),
                },
                &[&[&"mint".as_bytes()]],
            ),
            amount,
        )?;

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
    pub p_nft: Account<'info, TokenAccount>,
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
    pub token_metadata_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    #[account()]
    pub metadata: Account<'info, Metadata>,
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

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
