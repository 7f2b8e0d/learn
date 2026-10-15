pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8ToKn76BXQvyeKKKdGPyJc9SSCPLq6e7aKKkoAhoda3a");

#[derive(Accounts)]
pub struct Hello {}

#[derive(Accounts)]
pub struct Bye {}

#[program]
pub mod two_greetings {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }

    pub fn bye(ctx: Context<Bye>) -> Result<()> {
        msg!("Bye, world!");
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }
}
