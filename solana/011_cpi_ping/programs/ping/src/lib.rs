pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GB2eZ3S2NTA7hmC3M1SjzoiwJ1uyWMZcYrMWS5VKBkpt");

#[derive(Accounts)]
pub struct Ping<'info> {
    pub user: Signer<'info>,
}

#[program]
pub mod ping {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn ping(_ctx: Context<Ping>) -> Result<()> {
        msg!("ping");
        Ok(())
    }
}
