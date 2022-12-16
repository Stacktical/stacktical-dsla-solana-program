use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use rust_decimal::prelude::*;

use crate::constants::*;
use crate::program::Dsla;
use crate::state::sla::Sla;
use crate::state::{Governance, Lockup, SlaAuthority, SlaStatus};

/// Instruction to claim all rewards up to the latest available
/// eg. if current period is 5 and I have never claimed before, I will receive all rewards up to 4th period according to the status, leverage and deviation
#[derive(Accounts)]
pub struct WithdrawProvider<'info> {
    /// provider
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
    pub sla_authority: Account<'info, SlaAuthority>,

    /// The token account to claimer the money in
    #[account(mut, associated_token::mint=mint, associated_token::authority=withdrawer)]
    pub withdrawer_token_account: Box<Account<'info, TokenAccount>>,

    /// The token account with pt tokens
    #[account(mut, associated_token::mint=pt_mint, associated_token::authority=withdrawer)]
    pub withdrawer_pt_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            withdrawer.key().as_ref(),
            LOCKUP_PROVIDER_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        bump,
    )]
    pub pt_lockup: Box<Account<'info, Lockup>>,

    #[account(
        mut,
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
        mut,
        seeds = [
            PT_MINT_SEED.as_bytes(),
            sla.key().as_ref(),
        ],
        constraint = pt_mint.is_initialized == true,
        bump,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,

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

    // @todo test if effectvily only the `associated_token::authority` can be passed here
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = program_data.upgrade_authority_address.unwrap()
    )]
    pub protocol_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = sla.sla_deployer_address
    )]
    pub deployer_token_account: Box<Account<'info, TokenAccount>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawProvider<'info> {
    fn pt_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.pt_mint.to_account_info(),
                from: self.withdrawer_pt_account.to_account_info(),
                authority: self.withdrawer.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<WithdrawProvider>, pt_burn_amount: u64) -> Result<()> {
    let leverage = &ctx.accounts.sla.leverage.to_decimal();
    let pt_burn_amount_dec = Decimal::from_u64(pt_burn_amount).unwrap();
    let provider_pool_size_dec = Decimal::from_u128(ctx.accounts.sla.provider_pool_size).unwrap();
    let pt_supply_dec = Decimal::from_u128(ctx.accounts.sla.pt_supply).unwrap();
    let sla_status = ctx.accounts.sla.period_data.get_current_period_id()?;

    // REFRESH AVAILABLE TOKENS IN THE LOCKUPS
    ctx.accounts.pt_lockup.update_available_tokens(sla_status)?;

    // CALCULATIONS
    // @todo add test
    let tokens_to_withdraw = pt_burn_amount_dec
        .checked_div(provider_pool_size_dec.checked_div(pt_supply_dec).unwrap())
        .unwrap()
        .floor();
    let tokens_to_withdraw_u128 = tokens_to_withdraw.to_u128().unwrap();

    // @todo add error here
    match sla_status {
        SlaStatus::Ended => {
            // check tokens are avaialable
            require_gte!(ctx.accounts.sla.provider_pool_size, tokens_to_withdraw_u128);
        }
        _ => {
            // CHECK IF ENOUGH PROVIDER LIQUIDITY IS AVAILABLE FOR WITHDRAWAL
            let leverage_adjusted_user_pool = leverage
                .checked_mul(
                    Decimal::from_u128(
                        ctx.accounts
                            .sla
                            .user_pool_size
                            .checked_sub(tokens_to_withdraw_u128)
                            .unwrap(),
                    )
                    .unwrap(),
                )
                .unwrap()
                .floor();

            // @todo add error here
            require_gte!(provider_pool_size_dec, leverage_adjusted_user_pool);
        }
    }

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
    // @todo add test

    let provider_amount = tokens_to_withdraw
        .floor()
        .to_u64()
        .unwrap()
        .checked_sub(protocol_amount)
        .unwrap()
        .checked_sub(deployer_amount)
        .unwrap();

    // @todo add test
    token::burn(ctx.accounts.pt_burn_context(), pt_burn_amount)?;

    // @todo add test
    ctx.accounts.sla.pt_supply = ctx
        .accounts
        .sla
        .pt_supply
        .checked_sub(pt_burn_amount as u128)
        .unwrap();
    ctx.accounts.pt_lockup.withdraw(pt_burn_amount)?;

    let sla_key = ctx.accounts.sla.key().clone();
    let authority_bump = *ctx
        .bumps
        .get("sla_authority")
        .expect("sla_authority should exists");
    let seeds = &[
        SLA_AUTHORITY_SEED.as_bytes(),
        sla_key.as_ref(),
        &[authority_bump],
    ];

    let signer_seeds = &[&seeds[..]];
    let provider_transfer_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool.to_account_info(),
            to: ctx.accounts.withdrawer_token_account.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
        signer_seeds,
    );
    let deployer_transfer_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool.to_account_info(),
            to: ctx.accounts.deployer_token_account.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
        signer_seeds,
    );

    let protocol_transfer_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool.to_account_info(),
            to: ctx.accounts.protocol_token_account.to_account_info(),
            authority: ctx.accounts.sla_authority.to_account_info(),
        },
        signer_seeds,
    );

    // TRANSFER TOKENS
    token::transfer(provider_transfer_context, provider_amount)?;
    token::transfer(deployer_transfer_context, deployer_amount)?;
    token::transfer(protocol_transfer_context, protocol_amount)?;
    ctx.accounts.sla.provider_pool_size = ctx
        .accounts
        .sla
        .provider_pool_size
        .checked_sub(provider_amount as u128)
        .unwrap();
    Ok(())
}
