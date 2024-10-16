/*
    Example smart contract program for storing the user's preferences like
    favorite number, color, and hobbies. Storing data on the
    blockchain is not free. The amount of data you store affects
    the cost of transactions and the rent required to maintain the account.
*/

use anchor_lang::prelude::*;

/*
    Declare the program id on solana smart contract.
    Actually, it will automatically generate on `cargo clean` and `anchor build`.
*/
declare_id!("3a83mZacMxzkshfthmCwR7TaG7EP4m2phcE8FtaYdCrN");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // Type of account used by anchor for some of its checks

// Macro #[program] is for declare a full solana program or smart contract
#[program]
pub mod favorites {
    use super::*; // Import all the items from the parent module

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", context.program_id);

        let user_public_key = context.accounts.user.key();
        msg!(
            "User {}'s favorite number is {}, favorite color is {}, and hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
