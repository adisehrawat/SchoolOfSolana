//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
///
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // TODO: Implement deposit functionality
    let vault = &mut ctx.accounts.vault;
    let user = &ctx.accounts.user;

    require!(vault.locked == false, VaultError::VaultLocked);

    let user_balance = user.to_account_info().lamports();
    require!(user_balance >= amount, VaultError::InsufficientBalance);

    let instruction = transfer(&user.key(), &vault.key(), amount);

    invoke(&instruction,
         &[
        user.to_account_info(),
        vault.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ])?;
    emit!(DepositEvent {
        vault: vault.key(),
        user: user.key(),
        amount,
    });

    Ok(())
}
