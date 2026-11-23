use anchor_lang::prelude::*;

use crate::{error::ErrorCode, state::Task};

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    pub creator: Signer<'info>,
    #[account(
        mut,
        has_one = creator @ ErrorCode::Unauthorized,
    )]
    pub task: Account<'info, Task>,
}

pub fn handle_complete_task(ctx: Context<CompleteTask>) -> Result<()> {
    require!(
        !ctx.accounts.task.completed,
        ErrorCode::TaskAlreadyCompleted
    );
    ctx.accounts.task.completed = true;
    msg!("Tarea {} completada", ctx.accounts.task.id);
    Ok(())
}
