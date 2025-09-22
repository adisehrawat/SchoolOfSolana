//-------------------------------------------------------------------------------
///
/// TASK: Implement the initialize tweet functionality for the Twitter program
/// 
/// Requirements:
/// - Validate that topic and content don't exceed maximum lengths
/// - Initialize a new tweet account with proper PDA seeds
/// - Set tweet fields: topic, content, author, likes, dislikes, and bump
/// - Initialize counters (likes and dislikes) to zero
/// - Use topic in PDA seeds for tweet identification
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn initialize_tweet(
    ctx: Context<InitializeTweet>,
    topic: String,
    content: String,
) -> Result<()> {
    // TODO: Implement initialize tweet functionality
    let tweet = &mut ctx.accounts.tweet;
    require!(topic.len() <= 50, TwitterError::TopicTooLong);
    require!(content.len() <= 500, TwitterError::ContentTooLong);
    tweet.topic = topic;
    tweet.content = content;
    tweet.tweet_author = ctx.accounts.tweet_authority.key();
    tweet.likes = 0;
    tweet.dislikes = 0;
    tweet.bump = ctx.bumps.tweet;
    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct InitializeTweet<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub tweet_authority: Signer<'info>,
    #[account(
        init,
        payer = tweet_authority,
        space = 8 + Tweet::INIT_SPACE,
        seeds = [topic.as_bytes(), b"TWEET_SEED" , tweet_authority.key().as_ref()],
        bump,
    )]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
