use anchor_lang::prelude::*;

#[account]
pub struct CollectionPool {
    /// NFT Collection ID
    pub collection_id: Pubkey,

    ///Pool Owner
    pub pool_owner: Pubkey,

    /// Loan Duration
    pub duration : i64,

    /// Total Loans
    pub total_offers: u64,

    /// Bump
    pub bump: u8,
}


impl CollectionPool {

    /// Number of bytes to store a collection pool account
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1;
}