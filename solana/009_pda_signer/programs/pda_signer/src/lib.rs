pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CSUPLwoRrWPKrLgdvCg2Uw4HFFrsc79BkmKjG37HHr8Q");

#[dervie(Accounts)]
pub struct Withdraw {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        payer = author,
        space = 8 + Note::INIT_SPACE,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    
    #[account(mut)]
    pub recipient:SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[program]
pub mod pda_signer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> {
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
