use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod idsolana {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
    pub fn claim_username(ctx: Context<CreateUser>, username: String, bump: u8) -> ProgramResult {
        if username.len() >= UserAccount::MAX_USERNAME_LEN {
            return Err(ProgramError::InvalidArgument);
        }
        ctx.accounts.user.username = username.clone();
        ctx.accounts.user.bump = bump;
        ctx.accounts.user.authority = ctx.accounts.authority.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct UserAccount {
    pub username: String,
    pub authority: Pubkey,

    pub bump: u8,
}

impl UserAccount {
    const SPACE: usize = 8 + 1 + 32 + Self::MAX_USERNAME_LEN;

    const MAX_USERNAME_LEN: usize = 140;
}

#[derive(Accounts)]
#[instruction(username: String, bump: u8)]
pub struct CreateUser<'info> {
    #[account(
        init,
        seeds = [username.as_bytes()],
        bump = bump,
        payer = authority,
        space = UserAccount::SPACE,
        owner = *program_id
    )]
    user: Account<'info, UserAccount>,

    system_program: AccountInfo<'info>,

    #[account(mut, signer)]
    authority: AccountInfo<'info>,
}
