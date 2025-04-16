use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("VoteFeatur3s11111111111111111111111111111111");

#[program]
pub mod document_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.total_documents = 0;
        state.total_members = 0;
        Ok(())
    }

    pub fn add_member(ctx: Context<AddMember>, member: Pubkey) -> Result<()> {
        let member_account = &mut ctx.accounts.member_account;
        member_account.member = member;

        let state = &mut ctx.accounts.state;
        state.total_members += 1;
        Ok(())
    }

    pub fn initialize_document(
        ctx: Context<InitializeDocument>,
        hash: [u8; 32],
    ) -> Result<()> {
        let document = &mut ctx.accounts.document;
        document.hash = hash;
        document.votes_gray = 0;
        document.votes_blue = 0;
        document.votes_yellow = 0;
        document.votes_green = 0;
        document.votes_red = 0;
        document.total_votes = 0;

        let state = &mut ctx.accounts.state;
        state.total_documents += 1;
        Ok(())
    }

    pub fn vote_gray(ctx: Context<Vote>, is_true: bool) -> Result<()> {
        let document = &mut ctx.accounts.document;
        document.votes_gray += if is_true { 1 } else { -1 };
        document.total_votes += 1;
        Ok(())
    }

    pub fn vote_blue(ctx: Context<Vote>, is_true: bool) -> Result<()> {
        let document = &mut ctx.accounts.document;
        document.votes_blue += if is_true { 1 } else { -1 };
        document.total_votes += 1;
        Ok(())
    }

    pub fn vote_yellow(ctx: Context<Vote>, is_true: bool) -> Result<()> {
        let document = &mut ctx.accounts.document;
        document.votes_yellow += if is_true { 1 } else { -1 };
        document.total_votes += 1;
        Ok(())
    }

    pub fn vote_green(ctx: Context<Vote>, is_true: bool) -> Result<()> {
        let document = &mut ctx.accounts.document;
        document.votes_green += if is_true { 1 } else { -1 };
        document.total_votes += 1;
        Ok(())
    }

    pub fn vote_red(ctx: Context<Vote>, is_true: bool) -> Result<()> {
        let document = &mut ctx.accounts.document;
        document.votes_red += if is_true { 1 } else { -1 };
        document.total_votes += 1;
        Ok(())
    }

    pub fn expand_document(
        ctx: Context<ExpandDocument>,
        new_link: String,
        new_hash: [u8; 32],
        title: String,
    ) -> Result<()> {
        let expansion = &mut ctx.accounts.expansion;
        expansion.original_hash = ctx.accounts.document.hash;
        expansion.new_link = new_link;
        expansion.new_hash = new_hash;
        expansion.title = title;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub member_account: Account<'info, Member>,
    #[account(mut)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeDocument<'info> {
    #[account(init, payer = user, space = 8 + 32 + 5 * 4 + 4)]
    pub document: Account<'info, Document>,
    #[account(mut)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub document: Account<'info, Document>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExpandDocument<'info> {
    #[account(mut)]
    pub document: Account<'info, Document>,
    #[account(init, payer = user, space = 8 + 32 + 32 + 64 + 64)]
    pub expansion: Account<'info, DocumentExpansion>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GlobalState {
    pub total_documents: u64,
    pub total_members: u64,
}

#[account]
pub struct Member {
    pub member: Pubkey,
}

#[account]
pub struct Document {
    pub hash: [u8; 32],
    pub votes_gray: i32,
    pub votes_blue: i32,
    pub votes_yellow: i32,
    pub votes_green: i32,
    pub votes_red: i32,
    pub total_votes: u32,
}

#[account]
pub struct DocumentExpansion {
    pub original_hash: [u8; 32],
    pub new_hash: [u8; 32],
    pub new_link: String,
    pub title: String,
}
