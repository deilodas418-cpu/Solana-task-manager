pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3T8VSbh6k1sEStgASQ3epLhCqdP2dv4WxVyAD4RMPCqj");

#[program]
pub mod solana_task_manager {
    use super::*;

    /// Crea el perfil del usuario (una sola vez por wallet).
    pub fn initialize_profile(ctx: Context<InitializeProfile>) -> Result<()> {
        crate::instructions::initialize_profile::handle_initialize_profile(ctx)
    }

    /// Crea una tarea. El id es el contador actual del perfil.
    pub fn create_task(
        ctx: Context<CreateTask>,
        title: String,
        description: String,
    ) -> Result<()> {
        crate::instructions::create_task::handle_create_task(ctx, title, description)
    }

    /// Edita título/descripción. Solo el creador y solo si no está completada.
    pub fn update_task(
        ctx: Context<UpdateTask>,
        title: String,
        description: String,
    ) -> Result<()> {
        crate::instructions::update_task::handle_update_task(ctx, title, description)
    }

    /// Marca la tarea como completada. Solo el creador.
    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
        crate::instructions::complete_task::handle_complete_task(ctx)
    }

    /// Cierra la cuenta de la tarea y devuelve la renta al creador.
    /// Regla de negocio: SOLO si la tarea ya está completada.
    pub fn delete_task(ctx: Context<DeleteTask>) -> Result<()> {
        crate::instructions::delete_task::handle_delete_task(ctx)
    }
}
