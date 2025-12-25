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
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 14 ------------------------------
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
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 13 ------------------------------
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
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 12 ------------------------------
// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target ==system"}
//     };
//     Ok(())
// }
//----------------------------- 11 ------------------------------
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
//         msg!{"target == system"}
//     };
//     Ok(())

// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0;0];
//         let owner = system_program::ID;
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts = vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 10 ------------------------------

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
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }

// #[cfg(test)]

// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//                         &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 9 ------------------------------
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
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64 /1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
// #[cfg(test)]

// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 8 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
// #[cfg(test)]
// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result=process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 7 ------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
// }
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
//     if target_account.owner=&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }

// #[cfg(test)]
// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//                 let account=AccountInfo::new(
//                         &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
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
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 14 ------------------------------
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
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 13 ------------------------------
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
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 12 ------------------------------
// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target ==system"}
//     };
//     Ok(())
// }
//----------------------------- 11 ------------------------------
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
//         msg!{"target == system"}
//     };
//     Ok(())

// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0;0];
//         let owner = system_program::ID;
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts = vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 10 ------------------------------

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
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }

// #[cfg(test)]

// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//                         &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 9 ------------------------------
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
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64 /1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
// #[cfg(test)]

// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 8 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
// #[cfg(test)]
// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//         let account=AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result=process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
// //----------------------------- 7 ------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
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

// #[cfg(test)]
// mod test{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance(){
//         let program_id=Pubkey::new_unique();
//         let key=Pubkey::new_unique();
//         let mut lamports=2_500_000_000;
//         let mut data=vec![0;0];
//         let owner=system_program::ID;
//                 let account=AccountInfo::new(
//                         &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts= vec![account];
//          println!("\n--- STARTING SIMULATION ---");
//          let result= process_instruction(&program_id,&accounts,&[]);
//          println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 7 ------------------------------

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
//     let accounts_iter = &mut accounts.iter();
//     let target_account = next_account_info(accounts_iter)?;
//     let account_pubkey = target_account.key;
//     let lamports = target_account.lamports();
//     let sol_balance = (lamports as f64) / 1_000_000_000.0;
//     if target_account.owner == &system_program::ID {
//         msg! {
//             "target == system"
//         }
//     }
//     Ok(())
// }
// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     pub fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0;0];
//         let owner = system_program::ID;
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts = vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
//----------------------------- 6 ------------------------------

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
//     let sol_balance= lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 5 ------------------------------
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
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 4 ------------------------------
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
//     let lamports= target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 3 ------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     msg
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
//     let lamports= target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!{"target == system "}
//     };
//     Ok(())
// }
//----------------------------- 2 ------------------------------

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
//         msg!{"target == system"}
//     };
//     Ok(())
// }
//----------------------------- 1 ------------------------------
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

pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    _instruction_data:&[u8]
)->ProgramResult{
    let accounts_iter=&mut accounts.iter();
    let target_account=next_account_info(accounts_iter)?;
    let account_pubkey=target_account.key;
    let lamports=target_account.lamports();
    let sol_balance=lamports as f64/1_000_000_000.0;
    if target_account.owner==&system_program::ID{
        msg!{"target == system"}
    };
    Ok(())
}




#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    #[test]
    fn test_hello_balance() {
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