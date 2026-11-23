use anchor_lang::prelude::*;

use crate::{
    constants::{MAX_DESCRIPTION_LEN, MAX_TITLE_LEN, PROFILE_SEED, TASK_SEED},
    error::ErrorCode,
    state::{Profile, Task},
};

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        seeds = [PROFILE_SEED, creator.key().as_ref()],
        bump = profile.bump,
        constraint = profile.owner == creator.key() @ ErrorCode::Unauthorized,
    )]
    pub profile: Account<'info, Profile>,
    #[account(
        init,
        payer = creator,
        space = 8 + Task::INIT_SPACE,
        seeds = [TASK_SEED, creator.key().as_ref(), profile.task_count.to_le_bytes().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_task(
    ctx: Context<CreateTask>,
    title: String,
    description: String,
) -> Result<()> {
    require!(!title.trim().is_empty(), ErrorCode::EmptyTitle);
    require!(title.len() <= MAX_TITLE_LEN, ErrorCode::TitleTooLong);
    require!(
        description.len() <= MAX_DESCRIPTION_LEN,
        ErrorCode::DescriptionTooLong
    );

    let task = &mut ctx.accounts.task;
    task.creator = ctx.accounts.creator.key();
    task.id = ctx.accounts.profile.task_count;
    task.title = title;
    task.description = description;
    task.completed = false;
    task.created_at = Clock::get()?.unix_timestamp;
    task.bump = ctx.bumps.task;

    ctx.accounts.profile.task_count += 1;

    msg!("Tarea {} creada por {}", task.id, task.creator);
    Ok(())
}
