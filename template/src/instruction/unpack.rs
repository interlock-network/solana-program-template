/****************************************************************
 * Solana program template
 ****************************************************************/

#![allow(non_snake_case)]

use solana_program::{
        program_error::ProgramError,
    };

use crate::{
        error::TemplateError::InvalidInstruction,
        instruction::data::TemplateInstruction,
        utils::*,
    };

// this is example instruction_data 'unpack' implementation
// (not related to the Pack implementations for account states)
//
// it is customary to specify instruction type with leading tag
//
// tag is one byte, so we could have up to 256 instructions


impl TemplateInstruction {

    // Unpacks a byte buffer into a TemplateInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok( match tag {
            0 => Self::InstructionOne {
                dataNumberA: rest[0],
                dataStringA: rest[9..].to_vec(),
                dataNumberB: unpack_number_u64(&rest[1..9])?,
            },
            1 => Self::InstructionTwo {
                dataNumberA: rest[0],
                dataStringA: rest[1..].to_vec(),
            },
            2 => Self::InstructionThree {
                dataNumberC: unpack_number_u32(&rest[0..])?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}



