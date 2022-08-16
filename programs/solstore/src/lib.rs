use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod errors;

pub use instruction::*;
pub use state::*;
pub use errors::ErrorCode;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solstore {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // Done
    // Create an Admin Account that's going to store the Admin Config Info + the funds gets comming here

    
    // Allow Admin Account to create Coupons and Products

    // Allow Admin to create Loyalty Points

    // Allow All the User to create GiftCards

    // Create a Product Account which is gonna be gated and is managed by the admin
    
    // NFT Coupons are managed against the Product

    // NFT Gift Cards are managed against the Product

    // There is going to be a pay function that is going to swap from orca and tranfer the Funds to the merchant account
}

#[derive(Accounts)]
pub struct Initialize {}
