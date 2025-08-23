#![allow(unused)]
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    entrypoint,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum InstructionType {
    IncrementData(u32),
    DecrementData(u32),
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct CounterType {
    count: u32,
}

#[cfg(feature="solana")]
entrypoint!(counter_app);

fn counter_app(_program_id: &Pubkey, accounts: &[AccountInfo], instructions: &[u8]) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    
    let instruction = InstructionType::try_from_slice(& instructions)?;
    let mut counter = CounterType::try_from_slice(& acc.data.borrow())?;

    match instruction {
        InstructionType::IncrementData(value) => { counter.count += value; },
        InstructionType::DecrementData(value) => { counter.count -= value; },
    }

    counter.serialize(&mut *acc.data.borrow_mut())?;
    msg!("{}", counter.count);

    Ok(())
}
