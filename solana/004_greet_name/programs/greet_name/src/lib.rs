pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7KzVpDvx7CAk8fTDeYoasN4bjWNJyAp4cRCpcuaLRER5");

#[derive(Accounts)]
pub struct Greet {}

#[program]
pub mod greet_name {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn greet(_ctx: Context<Greet>, name: String) -> Result<()> {
        msg!("Hello, {}!", name);
        Ok(())
    }
}
