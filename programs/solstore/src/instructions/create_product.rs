use anchor_lang::prelude::*;
use crate::state::*;


#[derive(Accounts)]
#[instruction(random_hash: [u8;32])]
pub struct CreateProduct<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
  #[account(
    mut, 
    constraint = merchant_account.authority == signer.key(),
    seeds = ["MERCHANT".as_bytes(), signer.key().as_ref()], 
    bump = merchant_account.bump,
   )] 
   pub merchant_account: Account<'info, Merchant>,

   #[account(init,payer=signer,seeds=["product".as_bytes(),&random_hash,signer.key().as_ref()],bump,space=Product::LEN)]
   pub product_account: Account<'info,Product>,

  // Misc
  pub system_program: Program<'info, System>,
}



pub fn handler(ctx: Context<CreateProduct>) -> Result<()> {
    let product_account = &mut ctx.accounts.product_account;
    product_account.is_gated = false;
    product_account.merchant_authority = ctx.accounts.signer.key();
    product_account.bump =*ctx.bumps.get("product_account").unwrap();
    product_account.no_of_coupons = 0;
    product_account.gated_amount = 0;
    msg!("MERCHANT ACCOUNT CREATED");
    Ok(())
}