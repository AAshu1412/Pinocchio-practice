use pinocchio::{account_info::AccountInfo, instruction, program_error::ProgramError, pubkey::find_program_address, ProgramResult};

pub struct InitializeAccounts<'a>{
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

impl <'a> TryFrom<&'a [AccountInfo]> for InitializeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, _] = accounts else {
            return  Err(ProgramError::NotEnoughAccountKeys);
        };

        if !owner.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if vault.lamports().ne(&0) {
            return  Err(ProgramError::InvalidAccountData);
        }

        let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);

        if vault.key().ne(&vault_key){
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { owner, vault })

    }
}
