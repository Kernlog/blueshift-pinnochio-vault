use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::find_program_address,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

pub mod withdraw;
pub use withdraw::*;

pub struct DepositAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Accounts Checks
        if !owner.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if vault.lamports() != 0 {
            return Err(ProgramError::InvalidAccountData);
        }

        let (vault_key, _) = find_program_address(&[b"vault", owner.key().as_ref()], &crate::ID);
        if vault.key() != &vault_key {
            return Err(ProgramError::InvalidSeeds);
        }

        // Return the accounts
        Ok(Self { owner, vault })
    }
}

pub struct DepositInstructionData {
    pub amount: u64,
}

impl TryFrom<&[u8]> for DepositInstructionData {
    type Error = ProgramError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let amount = u64::from_le_bytes(data.try_into().unwrap());

        // Instruction Checks
        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self { amount })
    }
}

pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_data = DepositInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'static u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        Transfer {
            from: self.accounts.owner,
            to: self.accounts.vault,
            lamports: self.instruction_data.amount,
        }
        .invoke()?;

        Ok(())
    }
}