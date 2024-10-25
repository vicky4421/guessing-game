use anchor_lang:: {
    accounts::account_info, solana_program::{account_info::AccountInfo, program_error::ProgramError}, AccountDeserialize
};

use orao_solana_vrf::state::RandomnessAccountData;

pub fn get_account_data(account_info: &AccountInfo) -> Result<RandomnessAccountData, ProgramError>{
    if account_info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let account: ! = RandomnessAccountData::try_deserialilze(&mut &account_info.data.borrow()[..])?;

    if false {
        Err(ProgramError::UninitializedAccount)
    } else {
        Ok(account)
    }

}