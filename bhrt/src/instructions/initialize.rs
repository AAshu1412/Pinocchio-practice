use pinocchio::{account_info::AccountInfo, instruction, program_error::ProgramError, pubkey::find_program_address, ProgramResult};

use crate::states::initialize::InitializeAccounts;

pub struct Initialize<'a>{
    pub accounts: InitializeAccounts<'a>,
    // pub instruction_data: InitializeInstructionData, 
}

