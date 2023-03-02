use {
    anchor_lang::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{Token, TokenAccount, Mint},
};

#[derive(Accounts)]
pub struct InitAccount<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = if lookup.to_account_info().data_len() < 12 { 12 } else { lookup.to_account_info().data_len() },
        seeds = [delegate.key().as_ref()], bump,
    )]
    pub lookup: Account<'info, DelegatedLookup>,
    ///CHECK:
    pub delegate: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DelegateAuth<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,
    #[account(
        mut,
        mint::decimals = 0,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = payer,
    )]
    pub token: Account<'info, TokenAccount>,
    /// CHECK: delegate
    pub delegate: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [delegate.key().as_ref()], bump,
    )]
    pub lookup: Account<'info, DelegatedLookup>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FreezeThawMPL<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,
    #[account(
        mut,
        mint::decimals = 0,
        mint::freeze_authority = me.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = mint,
    )]
    pub token: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [payer.key().as_ref()], bump,
    )]
    pub lookup: Account<'info, DelegatedLookup>,
    #[account(
        constraint = meta_program.key() == mpl_token_metadata::ID
    )]
    /// CHECK: metadata program
    pub meta_program: UncheckedAccount<'info>,
    #[account(
        owner = mpl_token_metadata::ID,
        seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), mint.key().as_ref(), b"edition"], bump,
        seeds::program = mpl_token_metadata::ID
    )]
    /// CHECK: me account
    pub me: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DelegatedLookup {
    pub delegated: Vec<Pubkey>
}
