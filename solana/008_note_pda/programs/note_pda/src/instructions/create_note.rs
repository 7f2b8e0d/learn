use anchor_lang::prelude::*;

use crate::{constants::*, state::*};

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        init,
        payer = author,
        space = 8 + Note::INIT_SPACE,
        seeds = [NOTE_SEED, author.key().as_ref()],
        bump
    )]
    pub note: Account<'info, Note>,

    pub system_program: Program<'info, System>,
}