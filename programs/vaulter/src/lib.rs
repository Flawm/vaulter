pub mod state;

use {
    anchor_lang::prelude::*,
    crate::state::*,
    anchor_lang::solana_program::{
        program as spl,
    },
    mpl_token_metadata::instruction::{
        freeze_delegated_account, thaw_delegated_account
    }
};

declare_id!("VA1Ley3XMKGbtUx2MukbBiErRgD2UKLRoWPxooP7aKY");

#[program]
pub mod vaulter {
    use super::*;

    pub fn init(_ctx: Context<InitAccount>) -> Result<()> {
        Ok(())
    }

    pub fn give_authority(ctx: Context<DelegateAuth>) -> Result<()> {
        let lookup = &mut ctx.accounts.lookup;

        match lookup.delegated.iter().position(|x| x == &ctx.accounts.token.key()) {
            Some(_) => (),
            None => {
                let lookup_account = lookup.to_account_info();
                lookup_account.realloc(lookup_account.data_len() + 32, false)?;

                lookup.delegated.push(ctx.accounts.token.key());

                let rent = Rent::get()?;
                let rent_exemption: u64 = rent.lamports_per_byte_year * rent.exemption_threshold as u64;
                let amount = 32 * rent_exemption;

                anchor_lang::system_program::transfer(
                    CpiContext::new(ctx.accounts.system_program.to_account_info(), anchor_lang::system_program::Transfer {
                        from: ctx.accounts.payer.to_account_info().clone(),
                        to:   lookup_account.clone()
                    }),
                    amount
                )?;
            }
        }

        // transfer delegate authority over token account
        anchor_spl::token::approve(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), anchor_spl::token::Approve {
                to:        ctx.accounts.token.to_account_info(),
                delegate:  ctx.accounts.delegate.to_account_info(),
                authority: ctx.accounts.payer.to_account_info()
            }),
            1
        )
    }

    pub fn freeze_mpl(ctx: Context<FreezeThawMPL>) -> Result<()> {
        spl::invoke(
            &freeze_delegated_account(
                ctx.accounts.meta_program.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.token.key(),
                ctx.accounts.me.key(),
                ctx.accounts.mint.key(),
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.token.to_account_info(),
                ctx.accounts.me.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.meta_program.to_account_info(),
            ]
        )?;

        Ok(())
    }

    pub fn thaw_mpl(ctx: Context<FreezeThawMPL>) -> Result<()> {
        spl::invoke(
            &thaw_delegated_account(
                ctx.accounts.meta_program.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.token.key(),
                ctx.accounts.me.key(),
                ctx.accounts.mint.key(),
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.token.to_account_info(),
                ctx.accounts.me.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.meta_program.to_account_info(),
            ]
        )?;

        anchor_spl::token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), anchor_spl::token::Transfer {
                from:      ctx.accounts.token.to_account_info(),
                to:        ctx.accounts.token.to_account_info(),
                authority: ctx.accounts.payer.to_account_info()
            }),
            1
        )?;

        let lookup = &mut ctx.accounts.lookup;

        match lookup.delegated.iter().position(|x| x == &ctx.accounts.token.key()) {
            Some(pos) => {
                lookup.delegated.remove(pos);

                let lookup_account = lookup.to_account_info();
                lookup_account.realloc(lookup_account.data_len() - 32, false)?;

                let rent = Rent::get()?;
                let rent_exemption: u64 = rent.lamports_per_byte_year * rent.exemption_threshold as u64;
                let amount = 32 * rent_exemption;

                let mut payer  = ctx.accounts.payer.lamports.borrow_mut();
                let mut bank   = lookup_account.lamports.borrow_mut();

                **bank  -= amount;
                **payer += amount;
                Ok(())
            }
            None => Ok(()),
        }
    }
}
