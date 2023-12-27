use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub enum OfferStatus {
    Taken,
    NotTaken,
}


#[account]
pub struct Offer {
    /// Collection
    pub collection: Pubkey,

    /// Offer Amount
    pub offer_lamport_amount: u64,

    /// Repay Amount
    pub repay_lamport_amount: u64,

    /// Lender
    pub lender: Pubkey,

    /// Loan Taken:
    pub is_loan_taken: OfferStatus,

    /// Borrower
    pub borrower: Pubkey,

    ///Bump 
    pub bump: u8,
}


impl Offer {

    /// Length of the Offer Struct
    pub const LEN: usize =  8 + 32 + 8 + 8 + 32 + 1 + 32 + 1;

}