pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4YJpSKRiwDtAfccQmkwV2zQ18mpcHEzrbgrDwhGgMPrt");

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    #[account(mut)]
    pub to: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) ->Result<()> {
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(anchor_lang::system_program::ID, cpi_accounts);
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;
        msg!("transferred: {}", amount);
        Ok(())
    }
}
