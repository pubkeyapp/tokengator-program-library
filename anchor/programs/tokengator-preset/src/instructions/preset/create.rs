use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(args: CreatePresetArgs)]
pub struct CreatePreset<'info> {
    #[account(
      init,
      payer = authority,
      space = Preset::size(&[authority.key()], &args.minter_config.metadata_config),
      seeds = [
        PREFIX,
        PRESET,
        &args.name.as_bytes()
      ],
      bump
    )]
    pub preset: Account<'info, Preset>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create(ctx: Context<CreatePreset>, args: CreatePresetArgs) -> Result<()> {
    let preset = &mut ctx.accounts.preset;

    let authority = ctx.accounts.authority.key();

    // Creating pointer account
    preset.set_inner(Preset {
        bump: ctx.bumps.preset,
        authorities: vec![authority.key()],
        description: args.description,
        name: args.name,
        image_url: args.image_url,
        minter_config: args.minter_config,
    });

    preset.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreatePresetArgs {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub minter_config: MinterConfig,
}
