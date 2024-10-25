use anchor_lang::prelude::*;

pub mod misc;

use orao_solana_vrf::{
    cpi::accounts::RequestV2,
    program::OraoVrf,
    state::NetworkState,
    CONFIG_ACCOUNT_SEED,
    RANDOMNESS_ACCOUNT_SEED
};

declare_id!("4XsGy8pPM4wRHBXS37q7PbfUJMK3cY7yAez92MxhEvWi");

#[program]
pub mod guessing_game {
    use core::random;

    use misc::get_account_data;

    use super::*;

    pub fn initialize(ctx: Context<GuessingGame>, force_seed: [u8; 32]) -> Result<()> {

        orao_solana_vrf::cpi::request_v2(ctx.accounts.request_ctx(), force_seed);

        Ok(())
    }

    pub fn guess(ctx: Context<GuessingGame>, user_guess: u64, _force_seed: [u8; 32]) -> Result<()> {
        let account_data = get_account_data(&ctx.accounts.random)?;

        if let Some(randomness) = account_data.fulfilled_randomness() {
            // use the first 8 bytes from the byte slice
            let byte_array: [u8; 8] = randomness[0..size_of::<u64>()].try_into().unwrap();
            let secret_number = u64::from_le_bytes(byte_array);
            let secret_number = secret_number % 11;

            match user_guess.cmp(&secret_number) {
                Ordering::Less => msg!("Too small!"),
                Ordering::Greater => msg!("Too big!"),
                Ordering::Equal => msg!("You win! {:?}", secret_number),
            }
        }

        Ok(())
    }
}

impl<'info> GuessingGame<'info> {
    pub fn request_ctx($self) -> CpiContext<'_, '_, '_, 'info, RequestV2<'info>> {
        let cpi_program = self.orao_vrf.to_account_info();
        let cpi_accounts = RequestV2 {
            payer: self.payer.to_account_info(),
            network_state: self.network_state.to_account_info(),
            treasury: self.treasury.to_account_info(),
            request: self.random.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
#[instruction(user_guess: u64, force_seed: [u8; 32])]
pub struct GuessingGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED.as_ref()],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub network_state: Account<'info, NetworkState>,

    /// CHECK
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// CHECK
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &force_seed],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: AccountInfo<'info>,

    pub orao_vrf: Program<'info, OraoVrf>,
    pub system_program: Program<'info, System>,
}
