use anchor_lang::prelude::*;

pub mod states;

declare_id!("6FNXD8XxU7rrbe4Nw4Pmuh9aMyuzjPey3HdseWNuJhBr");

#[program]
pub mod nft_lend_borrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
