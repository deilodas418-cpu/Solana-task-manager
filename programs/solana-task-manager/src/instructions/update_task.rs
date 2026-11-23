use anchor_lang::prelude::*;

use crate::{
    constants::{MAX_DESCRIPTION_LEN, MAX_TITLE_LEN},
    error::ErrorCode,
    state::Task,
};

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        has_one = creator @ ErrorCode::Unauthorized,
    )]
    pub task: Account<'info, Task>,
}

pub fn handle_update_task(
    ctx: Context<UpdateTask>,
    title: String,
    description: String,
) -> Result<()> {
    require!(
        !ctx.accounts.task.completed,
        ErrorCode::TaskAlreadyCompleted
    );
    require!(!title.trim().is_empty(), ErrorCode::EmptyTitle);
    require!(title.len() <= MAX_TITLE_LEN, ErrorCode::TitleTooLong);
    require!(
        description.len() <= MAX_DESCRIPTION_LEN,
        ErrorCode::DescriptionTooLong
    );

    let task = &mut ctx.accounts.task;
    task.title = title;
    task.description = description;
    msg!("Tarea {} actualizada", task.id);
    Ok(())
}
