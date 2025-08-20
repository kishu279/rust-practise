// #![allow(unused)]
// use borsh::{to_vec, BorshDeserialize, BorshSerialize};
// use solana_program::{
//     account_info::{AccountInfo, next_account_info},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey
// };

// #[derive(BorshDeserialize, BorshSerialize)]
// enum InstructionType {
//     IncrementData(u32),
//     DecrementData(u32),
// }

// #[derive(BorshSerialize, BorshDeserialize)]
// struct Counter {
//     count: u32,
// }


// #[cfg(feature="solana")]
// entrypoint!(counter_application);

// fn counter_application(_programId: Pubkey, accounts: &[AccountInfo], instructions: &[u8]) -> ProgramResult {
//     let acc = next_account_info(&mut accounts.iter())?;

//     let instruction = InstructionType::try_from_slice(& instructions)?;
//     let mut count = Counter::try_from_slice(&acc.data.borrow())?;

//     match instruction {
//         InstructionType::IncrementData(value) => {
//             count.count += value;
//         }
//         InstructionType::DecrementData(value) => {
//             count.count -= value;
//         }
//     }

//     count.serialize(&mut *acc.data.borrow_mut())?;
//     msg!("{}", count.count);


//     Ok(())
// }


#![allow(unused)]
use borsh::{BorshSerialize, BorshDeserialize, to_vec};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    entrypoint,
    msg,
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
entrypoint!(counter);

fn counter (_publicId: &Pubkey, accounts: &[AccountInfo], instructions: &[u8]) -> ProgramResult {    
    
    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(& instructions)?;
    let mut count = CounterType::try_from_slice(&mut acc.data.borrow())?;

    match instruction_type {
        InstructionType::IncrementData(value) => {count.count += value;}
        InstructionType::DecrementData(value) => {count.count -= value;}
    }

    count.serialize(&mut *acc.data.borrow_mut())?;
    msg!("{}", count.count);
    
    Ok(())
}