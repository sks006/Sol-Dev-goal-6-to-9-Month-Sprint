// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_00_00_00_00.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target Id == system Id"}
//     };
//     Ok(())
// }

//------------------------------------------ 14 -----------------------------
// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
//     pubkey::Pubkey,
// };
// entrypoint! {
//     process_instruction
// }
// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system id"}
//     };
//     Ok(())
// }
//------------------------------------------ 13 ----------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey= target_account.key;
//     let lamports= target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system id"}
//     };
//     Ok(())

// }
//------------------------------------------ 12 -------------------------------------
// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
// };
// entrypoint! {
//     process_instruction
// }

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system id"}
//     };
//     Ok(())
// }
//------------------------------------------ 11 -------------------------------------

// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
//     system_program,
// };
// entrypoint! {
//     process_instruction
// }

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let accounts_iter= &mut accounts.iter();
//     let target_account= next_account_info(accounts_iter)?;
//     let account_pubkey= target_account.key;
//     let lamports= target_account.lamports();
//     let sol_balance= lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system "}
//     };
//     Ok(())
// }
//------------------------------------------ 10 -------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target -=- system id"}
//     };
//     Ok(())
// }

//------------------------------------------ 9 -------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//------------------------------------------ 8 -------------------------------------
// use solana_program::{
//     account_info::{next_account_info,AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey= target_account.key;
//     let lamports= target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system_program"}
//     };
//     Ok(())
// }
//------------------------------------------ 7 -------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system id"}
//     };
//     Ok(())
// }
//------------------------------------------ 6 -------------------------------------
// use solana_program::{
//     account_info::{next_account_info, AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     pubkey::Pubkey,
//     system_program,
// };

// entrypoint!(process_instruction);

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8],
// ) -> ProgramResult {
//     let accounts_iter = &mut accounts.iter();
    
//     // Get the target account
//     let target_account = next_account_info(accounts_iter)?;
    
//     // Validate account ownership (common security practice)
//     if target_account.owner != &system_program::ID {
//         return Err(ProgramError::IncorrectProgramId);
//     }
    
//     // Get account information
//     let account_pubkey = target_account.key;
//     let lamports = target_account.lamports();
//     let sol_balance = lamports as f64 / 1_000_000_000.0;
    
//     // Log information
//     msg!("Account: {}", account_pubkey);
//     msg!("Lamports: {}", lamports);
//     msg!("SOL Balance: {}", sol_balance);
//     msg!("Owner is System Program: true");
    
//     Ok(())
// }

//------------------------------------------ 5 -------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
    
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
//     program_error::ProgramError,
//     pubkey::Pubkey,

// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
// let accounts_iter=&mut accounts.iter();
// let target_account=next_account_info(accounts_iter)?;
// let account_pubkey=target_account.key;
// let lamports=target_account.lamports();
// let sol_balance=lamports as f64/1_000_000_000.0;
// if target_account.owner!=&system_program::ID{
//    return Err(ProgramError::IncorrectProgramId);
// };
//     msg!("Account: {}", account_pubkey);
//     msg!("Lamports: {}", lamports);
//     msg!("SOL Balance: {}", sol_balance);
//     msg!("Owner is System Program: true");
// Ok(())
// }
//------------------------------------------ 4 -------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//    program_id:&Pubkey,
//    accounts:&[AccountInfo],
//    _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance =lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     msg!("Account: {}", account_pubkey);
//     msg!("Lamports: {}", lamports);
//     msg!("SOL Balance: {}", sol_balance);
//     msg!("Owner is System Program: true");
//     Ok(())
// }

//------------------------------------------ 3 -------------------------------------
use solana_program::{
    account_info::{
    next_account_info,AccountInfo
    },
    pubkey::Pubkey,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    system_program
};
entrypoint!{process_instruction}

pub fn  process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    _instruction_data:&[]
)

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    #[test]
    fn testhello_balance() {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 2_500_000_000;
        let mut data = vec![0;0];
        let owner = system_program::ID;
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default()
        );
        let accounts = vec![account];
        println!("\n--- STARTING SIMULATION ---");
        let result = process_instruction(&program_id, &accounts, &[]);
        assert!(result.is_ok());
        println!("--- SIMULATION SUCCESSFUL ---\n");
    }
}
