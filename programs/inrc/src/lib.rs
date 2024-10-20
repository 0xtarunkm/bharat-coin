pub mod constants;
pub mod contexts;
pub mod error;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use contexts::*;
pub use state::*;

declare_id!("Az9osrJiK6y3gRhnBNuBL6TVyM4QNWJjmHytWLMFvaEc");

#[program]
pub mod inrc {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        liquidation_threshold: u64,
        liquidation_bonus: u64,
        min_health_factor: u64,
    ) -> Result<()> {
        ctx.accounts.init_config(
            liquidation_threshold,
            liquidation_bonus,
            min_health_factor,
            &ctx.bumps,
        )
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        ctx.accounts.update_config(min_health_factor)
    }

    pub fn deposit_collateral(
        ctx: Context<DepositCollateral>,
        collateral_amount: u64,
        mint_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .deposit_collateral(collateral_amount, mint_amount, &ctx.bumps)
    }

    pub fn withdraw_collateral(
        ctx: Context<WithdrawCollateral>,
        collateral_amount: u64,
        burn_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .redeem_collateral(collateral_amount, burn_amount)
    }

    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        ctx.accounts.liquidate(amount_to_burn)
    }
}
