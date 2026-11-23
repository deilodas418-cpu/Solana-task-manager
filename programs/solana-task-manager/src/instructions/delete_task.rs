use anchor_lang::prelude::*;

use crate::{error::ErrorCode, state::Task};

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        has_one = creator @ ErrorCode::Unauthorized,
        close = creator,
    )]
    pub task: Account<'info, Task>,
}

pub fn handle_delete_task(ctx: Context<DeleteTask>) -> Result<()> {
    // Regla de cierre condicionado: solo se puede cerrar (y recuperar la
    // renta) una tarea ya completada. Una tarea pendiente NO se puede borrar.
    require!(ctx.accounts.task.completed, ErrorCode::TaskNotCompleted);
    msg!("Tarea {} eliminada, renta devuelta a {}", ctx.accounts.task.id, ctx.accounts.creator.key());
    Ok(())
}
