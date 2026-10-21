pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("F6AiTLrLkTJrrTW5J5mEBVHg8bBvb5VXi5We1Zc4vJ1K");

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = payer,
        token::mint = mint,
        token::authority = owner,
    )]
    pub token_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    token_account: Account<'info, TokenAccount>,
    payer: Signer<'info>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    token_program: Program<'info, Token>,
}

#[program]
pub mod token_account {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn create_mint(_ctx: Context<CreateMint>) -> Result<()> {
        Ok(())
    }

    pub fn create_token_account(_ctx: Context<CreateTokenAccount>) -> Result<()> {
        Ok(())
    }

    pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}
