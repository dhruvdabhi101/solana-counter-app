use anchor_lang::prelude::*;

declare_id!("Hs2cnurfstPr7wvYzcJo1soZu6DPbewh6cFfzWqDHLvU");

#[program]
mod counter_solana {
    use super::*;

    pub fn initialize(ctx: Context<Create>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }
    pub fn update(ctx: Context<Increment>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16  ) ]
    pub base_account: Account<'info, BaseAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut) ]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>
}

#[account]
pub struct BaseAccount {
    pub count: u64,

}
