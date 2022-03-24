/****************************************************************
 * Solana program template
 ****************************************************************/

use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum TemplateError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,
    /// Try From Slice
    #[error("Try From Slice Fail")]
    TryFromSliceError,
    /// Account Creation Attempt Fail
    #[error("Account Creation Attempt Fail")]
    InstructionOneAttemptError,
}

impl From<TemplateError> for ProgramError {
    fn from(error: TemplateError) -> Self {
        ProgramError::Custom(error as u32)
    }
}
