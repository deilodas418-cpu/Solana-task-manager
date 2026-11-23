use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Solo el creador de la tarea puede realizar esta operación")]
    Unauthorized,
    #[msg("El título no puede estar vacío")]
    EmptyTitle,
    #[msg("El título supera los 64 caracteres")]
    TitleTooLong,
    #[msg("La descripción supera los 200 caracteres")]
    DescriptionTooLong,
    #[msg("La tarea ya está completada")]
    TaskAlreadyCompleted,
    #[msg("La tarea debe estar completada antes de poder eliminarla")]
    TaskNotCompleted,
}
