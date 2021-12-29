use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod wormhole_deposit_box {
    use super::*;

    // The relayers are in competition to open the deposit box
    // The deposit token account is a PDA of the bridgor and the mint, where the bridger can blindly send to based on his not funded wallet
    // Fee needs to be > DepositBox + ATA
    pub fn initialize_deposit_box(ctx: Context<InitializeDepositBox>) -> ProgramResult {
        // The relayer pays for the token account + the deposit box account
        let deposit_box = &mut ctx.accounts.deposit_box;

        deposit_box.relayer = ctx.accounts.relayer.key();
        deposit_box.relayer_redeem_expiry = Clock::get()?.unix_timestamp.checked_add(60).unwrap();

        Ok(())
    }

    // The redeem phase can done by anyone after expiry
    // The creation of the associated token account is the responsability of whoever redeems
    pub fn redeem(ctx: Context<Redeem>) -> ProgramResult {
        // TODO:
        // - Transfer tokens to bridgor ATA
        // - Transfer fees to relayer, how much fees?
        // - Transfer dust of SOL to bridger for subsequent txs

        // - Punish relayer by not giving him his fees if expiry is reached
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDepositBox<'info> {
    #[account(
        init,
        payer = relayer,
    )]
    deposit_box: Account<'info, DepositBox>,
    #[account(
        init,
        seeds = [bridger.key().as_ref(), token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = bridger,
        payer = relayer,
    )]
    deposit_token_account: Account<'info, TokenAccount>,
    token_mint: Account<'info, Mint>,
    bridger: UncheckedAccount<'info>,
    relayer: Signer<'info>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(
        mut,
        has_one = relayer,
        close = relayer,
    )]
    deposit_box: Account<'info, DepositBox>,
    #[account(
        mut,
        seeds = [bridger.key().as_ref(), token_mint.key().as_ref()],
        bump,
        close = relayer,
    )]
    deposit_token_account: Account<'info, TokenAccount>,
    token_mint: Account<'info, Mint>,
    bridger: UncheckedAccount<'info>,
    #[account(
        constraint = bridger_token_account.owner.key() == bridger.key()
    )]
    bridger_token_account: Account<'info, TokenAccount>,
    relayer: UncheckedAccount<'info>,
    redeemer: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct DepositBox {
    relayer: Pubkey,
    relayer_redeem_expiry: i64,
}
