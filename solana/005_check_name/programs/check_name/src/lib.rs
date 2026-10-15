pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7ZBDfrzVLiT8mVf8cPKbRDj72p2ESYrZMtn1uPxSTyxU");

#[derive(Accounts)]
pub struct Greet {}

#[program]
pub mod check_name {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn greet(_ctx: Context<Greet>, name: String) -> Result<()> {
        // if name.is_empty() {
        //     Err(error::ErrorCode::EmptyName.into())
        // } else {
        //     msg!("Hello, {}!", name);
        //     Ok(())
        // }

        require!(
            !name.is_empty(), 
            error::ErrorCode::EmptyName,
        );
        msg!("Hello, {}!", name);
        Ok(())
    }
}
