use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::rent::{Rent},
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token_2022::instructions::{InitializeMint, InitializeMint2};
use pinocchio_token_2022::ID as TOKEN_PROGRAM_ID;

pub struct InitializeAccounts<'a> {
    pub authority: &'a AccountInfo,
    pub program_state: &'a AccountInfo,
    pub bhrt_mint: &'a AccountInfo,
    pub bhrt_metadata: &'a AccountInfo,
    pub collection_mint: &'a AccountInfo,
    pub collection_token_account: &'a AccountInfo,
    pub metadata_program: &'a AccountInfo,
    pub collection_master_edition_account: &'a AccountInfo,
    pub nft_collection_metadata: &'a AccountInfo,
    pub instruction_sysvar: &'a AccountInfo,
    pub associated_token_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct ProgramState {
    pub authority: Pubkey,
    pub program_state_bump: u8,
    pub bhrt_mint_bump: u8,
    pub collection_mint_bump: u8,
    pub collection_metadata_bump: u8,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitializeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [authority, program_state, bhrt_mint, instruction_sysvar, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let rent = Rent::from_account_info(instruction_sysvar)?;

        // derive the canonical bump during account init
        let (derived_program_state_pda, bump) = find_program_address(&[b"program_state"], &crate::ID);
        if derived_program_state_pda.ne(program_state.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        //Signer Seeds
        let program_state_signer_seeds = [Seed::from(b"program_state"), Seed::from(&[bump])];
        let program_state_signers = [Signer::from(&program_state_signer_seeds[..])];

        CreateAccount {
            from: authority,
            to: program_state,
            space: core::mem::size_of::<ProgramState>() as u64,
            owner: &crate::ID,
            lamports: rent.minimum_balance(core::mem::size_of::<ProgramState>()),
        }
        .invoke_signed(&program_state_signers)?;

        

        InitializeMint2 {
            mint: bhrt_mint,
            decimals: 9,
            mint_authority: program_state.key(),
            freeze_authority: Some(program_state.key()),
            token_program: &TOKEN_PROGRAM_ID,
        }.invoke()?;



        // if !vault.is_owned_by(&pinocchio_system::ID) {
        //     return Err(ProgramError::InvalidAccountOwner);
        // }

        // if vault.lamports().ne(&0) {
        //     return  Err(ProgramError::InvalidAccountData);
        // }

        // let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);

        // if vault.key().ne(&vault_key){
        //     return Err(ProgramError::InvalidAccountOwner);
        // }

        Ok(Self { owner, vault })
    }
}
