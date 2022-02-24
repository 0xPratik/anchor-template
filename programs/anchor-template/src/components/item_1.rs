use anchor_lang::prelude::*;

// Data Manipulation / Logic
pub fn initialize(ctx: Context<Initialize>, bump: u8) -> ProgramResult {
    let counter = &mut ctx.accounts.counter;
    counter.data = 0;
    counter.signer = ctx.accounts.signer.key();
    counter.bump = bump;
    Ok(())
}

// Data Validation
#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct Initialize<'info> {
    #[account(init,seeds=[signer.key().as_ref()],bump=bump,payer=signer,space = 128)]
    pub counter: Account<'info, Counter>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// Data Structure
#[account]
pub struct Counter {
    pub signer: Pubkey,
    pub data: u64,
    pub bump: u8,
}
