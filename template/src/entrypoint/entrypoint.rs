/****************************************************************
 * Solana program template
 ****************************************************************/

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    entrypoint,
};
use crate::processor::run::Processor;

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::run_process(program_id, accounts, instruction_data)
}
