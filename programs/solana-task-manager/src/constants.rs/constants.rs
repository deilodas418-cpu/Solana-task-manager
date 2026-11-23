use anchor_lang::prelude::*;

#[constant]
pub const PROFILE_SEED: &[u8] = b"profile";

#[constant]
pub const TASK_SEED: &[u8] = b"task";

// Sin #[constant]: el macro de IDL de Anchor 1.2 no admite `usize`.
// Se usan solo en Rust (validación y #[max_len]).
pub const MAX_TITLE_LEN: usize = 64;
pub const MAX_DESCRIPTION_LEN: usize = 200;
