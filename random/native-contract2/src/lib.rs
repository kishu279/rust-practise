#![allow(unused)]
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    entrypoint::ProgramResult,
    entrypoint,
    pubkey::Pubkey
};

#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType {
    IncrementCount(u32),
    DecrementCount(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CounterType {
    count: u32,
}

#[cfg(feature="solana")]
entrypoint!(counter_application);

fn counter_application(_public_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let instruction = InstructionType::try_from_slice(& instruction_data)?;
    let mut counter = CounterType::try_from_slice(&mut acc.data.borrow())?;

    match instruction {
        InstructionType::IncrementCount(value) => {counter.count += value;},
        InstructionType::DecrementCount(value) => {counter.count -= value;},
    }

    counter.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Current counter is {}", counter.count);

    Ok(())
}
