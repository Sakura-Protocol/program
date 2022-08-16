use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Owner is not the Rent Vault")]
    InvalidTokenAccount,
}