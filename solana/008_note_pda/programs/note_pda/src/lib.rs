pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("oZiLqi9RpyshfCntaMoSsqbtpoBh8LmrhSQsyZjjPVA");

#[program]
pub mod note_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn create_note(ctx: Context<CreateNote>, message: String) -> Result<()> {
        ctx.accounts.note.author = ctx.accounts.author.key();
        ctx.accounts.note.message = message;
        msg!("note created");
        Ok(())
    }
}
