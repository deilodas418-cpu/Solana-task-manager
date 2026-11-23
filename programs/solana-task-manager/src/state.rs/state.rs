use anchor_lang::prelude::*;

use crate::constants::{MAX_DESCRIPTION_LEN, MAX_TITLE_LEN};

/// Perfil de un usuario: cuenta relacionada con todas sus tareas.
/// PDA: ["profile", owner]
#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub owner: Pubkey,
    /// Contador monotónico: el id de la próxima tarea.
    /// También es el componente que hace única cada PDA de tarea.
    pub task_count: u64,
    pub bump: u8,
}

/// Una tarea on-chain.
/// PDA: ["task", creator, id.to_le_bytes()]
#[account]
#[derive(InitSpace)]
pub struct Task {
    pub creator: Pubkey,
    pub id: u64,
    #[max_len(MAX_TITLE_LEN)]
    pub title: String,
    #[max_len(MAX_DESCRIPTION_LEN)]
    pub description: String,
    pub completed: bool,
    pub created_at: i64,
    pub bump: u8,
}
