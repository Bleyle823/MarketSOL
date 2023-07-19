use anchor_lang::prelude::*;

#[error_code]
pub enum BetError{
    #[msg("Cannot Enter the bet.")]
    CannotEnter,

    #[msg("Cannot claim.")]
    CannotClaim,

    #[msg("Cannot close.")]
    CannotClose,

    #[msg("Given key for the pyth account does not match")]
    InvalidPythKey,

    #[msg("Invalid Pyth Account")]
    InvalidPythAccount,

    #[msg("Price is too big to parse to u32")]
    PriceTooBig,
}