use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;

// In this File there are Utility functions which are frequently used through out the programs

pub fn spl_token_transfer<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
) -> ProgramResult {
    anchor_spl::token::transfer(
        CpiContext::new(
            token_program,
            Transfer {
                from,
                to,
                authority,
            },
        ),
        amount,
    )?;
    Ok(())
}

pub fn spl_token_transfer_signer<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    signer: &[&[&[u8]]],
    amount: u64,
) -> ProgramResult {
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            token_program,
            Transfer {
                from,
                to,
                authority,
            },
            signer,
        ),
        amount,
    )?;
    Ok(())
}
