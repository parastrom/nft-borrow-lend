use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub enum LiquidationStatus {
    HasMargin, 
    Liquidated,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub enum RepaymentStatus {
    NotYet,
    Repaid,
}


#[account]
pub struct ActiveLoan {

    /// Collection
    pub collection: Pubkey,

    /// Offer Account
    pub offer_account: Pubkey,

    /// Lender
    pub lender: Pubkey,

    /// Borrower
    pub borrower: Pubkey,


    /// NFT Mint,
    pub mint: Pubkey,

    /// Timestamp of loan taken
    pub loan_ts: u64,

    /// Repayment Timestamp
    pub repay_ts: u64,

    /// Repaid
    pub is_repaid: RepaymentStatus,
    
    /// Liquidated
    pub is_liquidated: LiquidationStatus,
    
    /// Bump
    pub bump: u8,
}


impl ActiveLoan {
    /// Length of Active Loan Struct
    pub const LEN: usize = 8 + 32 +  32 + 32 + 32 + 8 + 8 + 1 + 1 + 1;
}