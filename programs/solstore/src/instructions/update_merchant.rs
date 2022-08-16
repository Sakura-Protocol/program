use anchor_lang::prelude::*;
use crate::state::*;


#[derive(Accounts)]
pub struct UpdateMerchant<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
   #[account(
    mut, 
    constraint = merchant_account.authority == signer.key()
    seeds = ["MERCHANT".as_bytes(), signer.key().as_ref()], 
    bump = merchant_account.bump,
   )] 
   pub merchant_account: Account<'info, Merchant>,
}



pub fn handler(ctx: Context<UpdateMerchant>) -> Result<()> {
    let merchant_account = &mut ctx.accounts.merchant_account;
    merchant_account.authority = ctx.accounts.signer.key();
    merchant_account.bump =*ctx.bumps.get("merchant_account").unwrap();
    merchant_account.no_of_products = 0;
    msg!("MERCHANT ACCOUNT CREATED");
    Ok(())
}