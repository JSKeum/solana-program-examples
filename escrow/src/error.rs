use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}