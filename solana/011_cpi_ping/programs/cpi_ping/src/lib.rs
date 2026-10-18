pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ka87VMcW1tdvuXyYh8cUWxj5VsW4jEpvToLttVtJ2UV");

#[derive(Accounts)]
pub struct CallPing<'info> {
    pub user: Signer<'info>,
    pub ping_program: Program<'info, ping::program::Ping>,
}

#[program]
pub mod cpi_ping {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn cpi_ping(ctx: Context<CallPing>) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.ping_program.key(),
            ping::cpi::accounts::Ping {
                user: ctx.accounts.user.to_account_info(),
            },
        );
        ping::cpi::ping(cpi_ctx)?;
        msg!("called ping");
        Ok(())
    }
}
