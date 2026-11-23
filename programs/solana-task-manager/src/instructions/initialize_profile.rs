use anchor_lang::prelude::*;

use crate::{constants::PROFILE_SEED, state::Profile};

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Profile::INIT_SPACE,
        seeds = [PROFILE_SEED, owner.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_profile(ctx: Context<InitializeProfile>) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    profile.owner = ctx.accounts.owner.key();
    profile.task_count = 0;
    profile.bump = ctx.bumps.profile;
    msg!("Perfil creado para {}", profile.owner);
    Ok(())
}
