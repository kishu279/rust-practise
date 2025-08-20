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
entrypoint!(counter_app);

pub fn counter_app(_prodgram_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?; // take the mutable reference to the accounts iterator and at last it does optional to match the type

    let instruction_type = InstructionType::try_from_slice(& instruction_data)?;
    let mut counter = Counter::try_from_slice(&mut acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            counter.count += value;
        }
        InstructionType::Decrement(value) => {
            counter.count -= value;
        }
    }

    counter.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Counter updated to : {}", counter.count);    

    Ok(())
}
