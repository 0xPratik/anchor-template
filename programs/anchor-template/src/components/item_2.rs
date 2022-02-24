use crate::item_1::*;
use anchor_lang::prelude::*;

pub fn update(ctx: Context<Update>) -> ProgramResult {
    let counter = &mut ctx.accounts.counter;
    counter.data += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct Update<'info> {
    pub signer: Signer<'info>,
    #[account(mut,has_one=signer)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
