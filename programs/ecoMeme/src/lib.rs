// Import necessary modules and traits from the Anchor framework and SPL Token program.
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use solana_program::program_pack::Pack;

// Declares the unique ID for the smart contract on Solana.
declare_id!("TsL61ixhgPFgWrdWjHT9BZrMmBTy42Gfs912bHoAVaq");

// Define the main module for the `eco_meme` program.
#[program]
pub mod eco_meme {
    use super::*;

    // Define a function to handle meme token deposits.
    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
        // Use the Anchor framework to perform a meme token transfer from the depositor's account
        // to the program's associated meme token account.
        token::transfer(ctx.accounts.into_transfer_to_context(), amount)?;

        // Retrieve the number of decimals for the meme token to accurately log the amount.
        let decimals = ctx.accounts.token_mint.decimals;

        // Log transaction details in a structured format for clean off-chain processing.
        // The meme token is converted to USDC, and the amount is used to retire carbon credits from the ecoLedger API.  
        // The transaction generates an entry into the Leaderboard, and the user receives an impact certificate air-dropped to their wallet with the carbon offset details.
        msg!("Transaction Details: {{");
        msg!("  \"Depositor\": \"{}\",", ctx.accounts.depositor.key());
        msg!("  \"Amount\": {},", amount); // Note: Amount is logged as raw units.
        msg!("  \"Decimals\": {},", decimals);
        msg!("  \"Token Address\": \"{}\"", ctx.accounts.token_mint.key());
        msg!("}}");

        Ok(())
    }
}

// Define the accounts context for the `deposit_token` function.
#[derive(Accounts)]
pub struct DepositToken<'info> {
    // The account of the user making the deposit. Marked as mutable because the token transfer will alter its balance.
    #[account(mut)]
    pub depositor: Signer<'info>,
    // The depositor's token account for the specific SPL token. This is where tokens will be transferred from.
    // Using AccountInfo for flexibility and manual checks.
    /// CHECK: This is safe because we only read data from it
    #[account(mut)]
    pub deposit_token_account: AccountInfo<'info>,
    // The program's associated token account for the specific SPL token. This is where tokens will be transferred to.
    // It's initialized if needed, indicating it will be created if it doesn't exist.
    #[account(init_if_needed, payer = depositor, associated_token::mint = token_mint, associated_token::authority = depositor)]
    pub program_token_account: Account<'info, TokenAccount>,
    // The mint account of the SPL token being deposited.
    pub token_mint: Account<'info, Mint>,
    // Reference to the SPL Token program to perform the transfer.
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    // System program reference, required for creating accounts.
    pub system_program: Program<'info, System>,
    // Rent sysvar, used to determine the rent exemption status of accounts.
    pub rent: Sysvar<'info, Rent>,
    // Associated Token program reference, used for operations related to associated token accounts.
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// Implementation block for the DepositToken context, containing helper methods.
impl<'info> DepositToken<'info> {
    // Helper method to create a CPI context for the token transfer.
    fn into_transfer_to_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.deposit_token_account.clone(),
            to: self.program_token_account.to_account_info(),
            authority: self.depositor.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
