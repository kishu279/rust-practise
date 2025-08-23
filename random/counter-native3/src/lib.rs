#![allow(unused)]
use borsh::{BorshSerialize, BorshDeserialize, to_vec};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    entrypoint,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType {
    IncrementData(u32),
    DecrementData(u32),
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
    let counter = CounterType::try_from_slice(&mut acc.data.borrow())?;

    match instruction {
        InstructionType::IncrementData(value) => {counter.count += value;},
        InstructionType::DecrementData(value) => {counter.count -= value;},
    }

    counter.serialize(&mut *acc.data.borrow_mut())?;
    msg!("{}", counter.count);

    Ok(())
}
