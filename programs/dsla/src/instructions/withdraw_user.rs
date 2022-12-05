use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::program::Dsla;
use crate::state::sla::Sla;
use crate::state::{Governance, Lockup};

/// Instruction to claim all rewards up to the latest available
/// eg. if current period is 5 and I have never claimed before, I will receive all rewards up to 4th period according to the status, leverage and deviation
#[derive(Accounts)]
pub struct WithdrawUser<'info> {
    /// user
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    /// the SLA
    #[account(mut)]
    pub sla: Account<'info, Sla>,

    #[account(
        mut,
        seeds = [SLA_AUTHORITY_SEED.as_bytes(),sla.key().as_ref()],
        bump,
    )]
    pub sla_authority: SystemAccount<'info>,

    /// The token account to claimer the money in
    #[account(mut, associated_token::mint=mint, associated_token::authority=withdrawer)]
    pub withdrawer_token_account: Box<Account<'info, TokenAccount>>,

    /// The token account with ut tokens
    #[account(mut, associated_token::mint=ut_mint, associated_token::authority=withdrawer)]
    pub withdrawer_ut_account: Box<Account<'info, TokenAccount>>,

    // @fixme make sure mint is same as defined in initialization
    #[account(
        constraint = mint.is_initialized == true,
        constraint = mint.key() == sla.mint_address,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [POOL_SEED.as_bytes(), sla.key().as_ref()],
        token::mint=mint,
        token::authority=sla_authority,
        bump,
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            UT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        constraint = ut_mint.is_initialized == true,
        bump,
    )]
    pub ut_mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [
            withdrawer.key().as_ref(),
            LOCKUP_USER_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub ut_lockup: Box<Account<'info, Lockup>>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = sla.sla_deployer_address
    )]
    pub deployer_token_account: Box<Account<'info, TokenAccount>>,

    // @todo test if effectvily only the `associated_token::authority` can be passed here
    #[account(
            associated_token::mint = mint,
            associated_token::authority = program_data.upgrade_authority_address.unwrap()
        )]
    pub protocol_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [GOVERNANCE_SEED.as_bytes()],
        bump
    )]
    pub governance: Account<'info, Governance>,
    pub token_program: Program<'info, Token>,
    #[account(address = crate::ID)]
    pub program: Program<'info, Dsla>,
    // @fixme this need to be checked, that only allowed program_data is the one linked to the program
    pub program_data: Account<'info, ProgramData>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawUser<'info> {
    fn ut_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.ut_mint.to_account_info(),
                from: self.withdrawer_ut_account.to_account_info(),
                authority: self.withdrawer.to_account_info(),
            },
        )
    }
    fn user_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool.to_account_info(),
                to: self.withdrawer_token_account.to_account_info(),
                authority: self.sla_authority.to_account_info(),
            },
        )
    }
    fn deployer_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool.to_account_info(),
                to: self.deployer_token_account.to_account_info(),
                authority: self.sla_authority.to_account_info(),
            },
        )
    }
    fn protocol_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool.to_account_info(),
                to: self.protocol_token_account.to_account_info(),
                authority: self.sla_authority.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<WithdrawUser>, burn_amount: u64) -> Result<()> {
    let burn_amount_dec = Decimal::from_u64(burn_amount).unwrap();
    let user_pool_size_dec = Decimal::from_u128(ctx.accounts.sla.user_pool_size).unwrap();
    let ut_supply_dec = Decimal::from_u128(ctx.accounts.sla.ut_supply).unwrap();
    let period_id = ctx.accounts.sla.period_data.get_current_period_id()?;

    ctx.accounts.ut_lockup.update_available_tokens(period_id)?;

    // @todo add test
    let tokens_to_withdraw = burn_amount_dec
        .checked_div(user_pool_size_dec.checked_div(ut_supply_dec).unwrap())
        .unwrap()
        .floor();
    let tokens_to_withdraw_u128 = tokens_to_withdraw.to_u128().unwrap();

    // @todo add error
    require_gte!(ctx.accounts.sla.user_pool_size, tokens_to_withdraw_u128);

    // @todo add test
    let deployer_amount = tokens_to_withdraw
        .checked_mul(
            ctx.accounts
                .governance
                .sla_deployer_rewards_rate
                .to_decimal(),
        )
        .unwrap()
        .floor()
        .to_u64()
        .unwrap();
    // @todo add test
    let protocol_amount = tokens_to_withdraw
        .checked_mul(ctx.accounts.governance.protocol_rewards_rate.to_decimal())
        .unwrap()
        .floor()
        .to_u64()
        .unwrap();
    let user_amount = tokens_to_withdraw
        .to_u64()
        .unwrap()
        .checked_sub(protocol_amount)
        .unwrap()
        .checked_sub(deployer_amount)
        .unwrap();

    // @todo add test
    // BURN TOKENS
    token::burn(ctx.accounts.ut_burn_context(), burn_amount)?;
    ctx.accounts.sla.ut_supply = ctx
        .accounts
        .sla
        .ut_supply
        .checked_sub(burn_amount as u128)
        .unwrap();

    ctx.accounts.ut_lockup.withdraw(burn_amount)?;

    // @todo add test
    token::transfer(ctx.accounts.deployer_transfer_context(), deployer_amount)?;
    // @todo add test
    token::transfer(ctx.accounts.protocol_transfer_context(), protocol_amount)?;
    // @todo add test
    token::transfer(ctx.accounts.user_transfer_context(), user_amount)?;

    ctx.accounts.sla.user_pool_size = ctx
        .accounts
        .sla
        .user_pool_size
        .checked_sub(tokens_to_withdraw_u128)
        .unwrap();

    Ok(())
}
