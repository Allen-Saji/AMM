pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("JCwLFmShANVSrGa8jgvUyMYeSyDxbYRLHBHdPnprKiTF");

#[program]
pub mod anchor_amm {
    use super::*;
    pub fn initialize(ctx: Context<InitializeConfig>, seed: u64, fee: u16) -> Result<()> {
        ctx.accounts.initialize_config(seed, fee, ctx.bumps)
    }
    pub fn swap(ctx: Context<Swap>, is_x: bool, amount: u64, min: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min)
    }
    pub fn deposit(
        ctx: Context<Deposit>, 
        amount: u64, 
        max_x: u64, 
        max_y: u64
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn update(ctx: Context<UpdateConfig>, lock: bool) -> Result<()> {
        if lock {
            ctx.accounts.lock()
        } else {
            ctx.accounts.unlock()
        }
    }
}
