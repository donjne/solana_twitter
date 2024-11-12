use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod contexts;
pub mod states;

pub use contexts::*;

declare_id!("yXKM6CEkkzF5QJLyHZ1Zh77o23wjrCYjvSuHA2DKY6e");

#[program]
pub mod ackee_task_4 {
    use super::*;

    pub fn create_tweet(ctx: Context<CreateTweet>, topic: String, content: String) -> Result<()> {
        let bump = ctx.bumps.tweet;
        ctx.accounts.create(bump, topic, content)
    }

    pub fn delete_tweet(ctx: Context<DeleteTweet>) -> Result<()> {
        ctx.accounts.delete()
    }

    pub fn add_reaction(ctx: Context<AddReaction>) -> Result<()> {
        let bump = ctx.bumps.reaction;
        ctx.accounts.add(bump)
    }

    pub fn remove_reaction(ctx: Context<RemoveReaction>) -> Result<()> {
        ctx.accounts.remove()
    }

    pub fn add_comment(ctx: Context<AddComment>, content: String) -> Result<()> {
        let bump = ctx.bumps.comment;
        ctx.accounts.add(bump, content)
    }

    pub fn remove_comment(ctx: Context<RemoveComment>) -> Result<()> {
        ctx.accounts.remove()
    }
}


