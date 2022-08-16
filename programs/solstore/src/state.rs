use anchor_lang::prelude::*;


#[account]
pub struct Merchant {
    pub authority: Pubkey,
    pub bump: u8,
    pub no_of_products: u64,
    pub loyalty_token_mint: Option<Pubkey>,
}

impl Merchant {
    pub const LEN: usize = 8 + 32 + std::mem::size_of::<Self>();
}


#[account]
pub struct Product {
    pub merchant_authority: Pubkey,
    pub is_gated: bool,
    pub gated_mint: Option<Pubkey>,
    pub gated_amount: u64,
    pub no_of_coupons: u64,
    pub bump: u8,
    pub random_hash: [u8; 32],
}

impl Product {
    pub const LEN: usize = 8 + 32+ 32 + std::mem::size_of::<Self>();
}