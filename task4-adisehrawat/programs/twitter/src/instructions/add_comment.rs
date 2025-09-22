//-------------------------------------------------------------------------------
///
/// TASK: Implement the add comment functionality for the Twitter program
///
/// Requirements:
/// - Validate that comment content doesn't exceed maximum length
/// - Initialize a new comment account with proper PDA seeds
/// - Set comment fields: content, author, parent tweet, and bump
/// - Use content hash in PDA seeds for unique comment identification
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    
    require!(comment_content.len() <= 500, TwitterError::CommentTooLong);

    let comment = &mut ctx.accounts.comment;
    let tweet = &mut ctx.accounts.tweet;

    comment.content = comment_content;
    comment.comment_author = ctx.accounts.comment_author.key();
    comment.parent_tweet = tweet.key();
    comment.bump = ctx.bumps.comment;
    Ok(())
}
pub fn hash_string_content(content: &str) -> [u8; 32] {
    let hash = hash(content.as_bytes());
    hash.to_bytes()
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    #[account(
        init,
        payer = comment_author,
        seeds = [
            b"COMMENT_SEED",
            comment_author.key().as_ref(),
            &hash_string_content(&comment_content),
            tweet.key().as_ref(),
        ],
        bump,
        space = 8 + Comment::INIT_SPACE,
    )]
    pub comment: Account<'info, Comment>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}