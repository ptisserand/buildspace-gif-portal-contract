use anchor_lang::prelude::*;

declare_id!("Dt8bpxueHcsn35VdN1EoYgFCaS2tJcP7YodZyMMby6Se");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // get a reference to account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // let build the struct
        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // add it to the gif links list
        base_account.gif_list.push(item);
        base_account.total_gifs +=1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer=user, space=9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// specify which data we want for add_gif
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
