//-------------------------------------------------------------------------------
///
/// TASK: Implement the add reaction functionality for the Twitter program
/// 
/// Requirements:
/// - Initialize a new reaction account with proper PDA seeds
/// - Increment the appropriate counter (likes or dislikes) on the tweet
/// - Set reaction fields: type, author, parent tweet, and bump
/// - Handle both Like and Dislike reaction types
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction: ReactionType) -> Result<()> {
    // TODO: Implement add reaction functionality
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;
    let tweet = &mut ctx.accounts.tweet;
    let reaction_author_key = ctx.accounts.reaction_author.key();
    let tweet_key = tweet.key();
    let reaction_bump = ctx.bumps.tweet_reaction;
    require!(tweet.likes < u64::MAX, TwitterError::MaxLikesReached);
    require!(tweet.dislikes < u64::MAX, TwitterError::MaxDislikesReached);
    match reaction {
        ReactionType::Like => {
            tweet.likes += 1;
        }
        ReactionType::Dislike => {
            tweet.dislikes += 1;
        }
    }

    tweet_reaction.parent_tweet = tweet_key;
    tweet_reaction.reaction_author = reaction_author_key;
    tweet_reaction.reaction = reaction;
    tweet_reaction.bump = reaction_bump;

    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        init,
        payer = reaction_author,
        seeds = [b"TWEET_REACTION_SEED", reaction_author.key().as_ref(), tweet.key().as_ref()],
        bump,
        space = 8 + Reaction::INIT_SPACE
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
