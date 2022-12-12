use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("88pnQED3oR6Vb4ZpDCrzVjsw55nUgvx2Lprv1RkP1Nhy");

#[program]
pub mod myepicproject {
    // use anchor_lang::{accounts::account_info, solana_program::{entrypoint::ProgramResult, entrypoint_deprecated::ProgramResult}};

    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link,
            user_address: user.key(),
            votes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn upvote(ctx: Context<Upvote>, idx: u16) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        // let user = &mut ctx.accounts.user;

        let li = &base_account.gif_list;
        let idx: usize = idx.into();
        let orig = li[idx].to_owned();

        let item = ItemStruct {
            votes: orig.votes + 1,
            ..orig
        };

        base_account.gif_list[idx] = item;

        Ok(())
    }

    pub fn send_sol(ctx: Context<SendSol>, lamports: u64) -> ProgramResult {
        let from = &ctx.accounts.from;
        let to = &ctx.accounts.to;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &from.key(),
            &to.key(),
            lamports,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[from.to_account_info(), to.to_account_info()],
        )
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Upvote<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    // #[account(mut)]
    // pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    from: Signer<'info>,

    #[account(mut)]
    to: AccountInfo<'info>,

    system_program: Program<'info, System>,
}
