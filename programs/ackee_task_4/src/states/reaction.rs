use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Reaction {
    pub user: Pubkey,
    pub tweet: Pubkey,
    pub bump: u8,
}