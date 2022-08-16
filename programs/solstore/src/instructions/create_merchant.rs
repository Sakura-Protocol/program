use anchor_lang::prelude::*;
use crate::state::*;


#[derive(Accounts)]
pub struct CreateMerchant<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
   #[account(
    init, 
    payer = signer, 
    space = Merchant::LEN, 
    seeds = ["MERCHANT".as_bytes(), signer.key().as_ref()], 
    bump,
   )] 
   pub merchant_account: Account<'info, Merchant>,
  // Misc
  pub system_program: Program<'info, System>,
}



pub fn handler(ctx: Context<CreateMerchant>) -> Result<()> {
    let merchant_account = &mut ctx.accounts.merchant_account;
    merchant_account.authority = ctx.accounts.signer.key();
    merchant_account.bump =*ctx.bumps.get("merchant_account").unwrap();
    merchant_account.no_of_products = 0;
    msg!("MERCHANT ACCOUNT CREATED");
    Ok(())
}