use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("3zaQ9MAgTXR971Uc9u42G59GiwzMdfvD9x4VhCNuNg2Y");

#[program]
pub mod bit_write {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        new_bio: Vec<u8>,
    ) -> ProgramResult {
        let b_p_a = &mut ctx.accounts.tweet_account;
        b_p_a.authority = *ctx.accounts.authority.key;
        b_p_a.bio = new_bio.to_vec();
        let bio = from_utf8(&new_bio)
            .map_err(|err| {
                msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
                ProgramError::InvalidInstructionData
            })?;
        msg!(bio);
        Ok(())
    }

    pub fn make_tweet(
        ctx: Context<MutateAccount>,
        new_tweet: Vec<u8>,
    ) -> ProgramResult {
        let post = from_utf8(&new_tweet)
            .map_err(|err| {
                msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
                ProgramError::InvalidInstructionData
            })?;
        msg!(post); // loggin post, so  we can grab from the transaction block data

        let b_acc = &mut ctx.accounts.tweet_account;
        b_acc.latest_tweet = new_tweet; // save the latest post in the account.

        Ok(())
    }
    pub fn update_bio(
        ctx: Context<MutateAccount>,
        new_bio: Vec<u8>,
    ) -> ProgramResult {
        let b_acc = &mut ctx.accounts.tweet_account;
        b_acc.bio = new_bio;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 566 + 256)]
    pub tweet_account: Account<'info, TweetAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MutateAccount<'info> {
    #[account(mut, has_one = authority)]
    pub tweet_account: Account<'info, TweetAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct TweetAccount {
    pub authority: Pubkey,
    pub latest_tweet: Vec<u8>,
    pub bio: Vec<u8>,
}
