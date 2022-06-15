use chainlink_solana as chainlink;

#[derive(Accounts)]
pub struct GetSli<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: We're reading data from this chainlink feed account
    pub chainlink_feed: AccountInfo<'info>,
    /// CHECK: This is the Chainlink program library
    pub chainlink_program: AccountInfo<'info>,
    pub sli: Account<'info, Sli>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Execute>) -> Result<()> {
    let round = chainlink::latest_round_data(
        ctx.accounts.chainlink_program.to_account_info(),
        ctx.accounts.chainlink_feed.to_account_info(),
    )?;

    let decimals = chainlink::decimals(
        ctx.accounts.chainlink_program.to_account_info(),
        ctx.accounts.chainlink_feed.to_account_info(),
    )?;
    // write the latest price to the program output
    let sli = Sli::new(round.answer, u32::from(decimals));
    Ok(())
}
