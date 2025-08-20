#![allow(unused)]
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        entrypoint
};

#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

#[cfg(feature="solana")]
entrypoint!(counter);

pub fn counter(_program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(&instruction_data)?; // serializing

    let mut count = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            count.count += value;
        }
        InstructionType::Decrement(value) => {
            count.count -= value;
        }
    };

    count.serialize(&mut *acc.data.borrow_mut())?;

    msg!("Counter updated to : {}", count.count);
    
    Ok(())
}

