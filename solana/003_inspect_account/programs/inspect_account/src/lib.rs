pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8sTQknX2Kx9AKaaVouTguUAykKsqx3NiZMHtbPVsB9Jq");

#[derive(Accounts)]
pub struct Inspect<'info> {
    ///CHECK:只读账户信息，不修改
    pub target: UncheckedAccount<'info>,
}


#[program]
pub mod inspect_account {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn inspect(ctx: Context<Inspect>) -> Result<()> {
        msg!("key: {}", ctx.accounts.target.key());
        msg!("lamports: {}", ctx.accounts.target.lamports());
        msg!("owner: {}", ctx.accounts.target.owner);
        msg!("data_len: {}", ctx.accounts.target.data_len());
        Ok(())
    }
}
