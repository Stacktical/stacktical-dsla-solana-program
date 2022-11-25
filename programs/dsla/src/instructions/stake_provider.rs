use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::events::StakedProviderSideEvent;
use crate::state::sla::{Sla, SlaAuthority};
use crate::state::Lockup;

/// Instruction to stake on both sides
#[derive(Accounts)]
pub struct StakeProvider<'info> {
    // provide or user
    #[account(mut)]
    pub staker: Signer<'info>,
    pub sla: Account<'info, Sla>,

    #[account(
        mut,
        seeds = [sla.key().as_ref()],
        bump = sla.authority_bump_seed[0],
    )]
    pub sla_authority: Account<'info, SlaAuthority>,

    #[account(
        mut,
        seeds = [POOL_SEED.as_bytes(), sla.key().as_ref()],
        bump,
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            PT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,

    /// The account to claim the money from
    pub staker_token_account: Box<Account<'info, TokenAccount>>,

    /// pt tokens
    #[account(mut)]
    pub staker_pt_account: Box<Account<'info, TokenAccount>>,

    #[account(
        // @fixme this needs to be removed and used init
        init_if_needed,
        space = Lockup::LEN,
        payer = staker,
        seeds = [
            staker.key().as_ref(),
            LOCKUP_PROVIDER_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub pt_lockup: Box<Account<'info, Lockup>>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakeProvider<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.staker_token_account.to_account_info(),
                to: self.pool.to_account_info(),
                authority: self.staker.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<StakeProvider>, token_amount: u64) -> Result<()> {
    let token_amount_dec = Decimal::from_u64(token_amount).unwrap();
    let provider_pool_size_dec = Decimal::from_u128(ctx.accounts.sla.provider_pool_size).unwrap();
    let pt_supply_dec = Decimal::from_u128(ctx.accounts.sla.pt_supply).unwrap();

    // @fixme deal with stake before validation and withdrawals after

    // @todo add test for this
    let tokens_to_mint = token_amount_dec
        .checked_div(provider_pool_size_dec.checked_div(pt_supply_dec).unwrap())
        .unwrap()
        .floor()
        .to_u64()
        .unwrap();

    token::transfer(ctx.accounts.transfer_context(), token_amount)?;
    let sla = &mut ctx.accounts.sla;

    // @todo add test for this
    sla.provider_pool_size += token_amount as u128;

    let auth_seed = sla.authority_seed;
    let seeds = &[auth_seed.as_ref(), &sla.authority_bump_seed];
    let signer = &[&seeds[..]];

    let mint_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            to: ctx.accounts.staker_pt_account.to_account_info(),
            mint: ctx.accounts.pt_mint.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
        signer,
    );

    token::mint_to(mint_context, tokens_to_mint)?;
    // @todo add test for this
    sla.pt_supply += tokens_to_mint as u128;

    let lockup = &mut ctx.accounts.pt_lockup;
    let period_id = ctx.accounts.sla.period_data.get_current_period_id()?;

    lockup.stake_update(tokens_to_mint, period_id)?;

    // @todo improve this event
    emit!(StakedProviderSideEvent { token_amount });
    Ok(())
}
