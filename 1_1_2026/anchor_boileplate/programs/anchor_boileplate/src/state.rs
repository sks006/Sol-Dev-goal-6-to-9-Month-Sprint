// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u8,
// }
// impl UserVault{
//     //Manual space calculation(8 byte discriminator + fields)
//     pub const LEN:usize=8+32+8+8+1;
// }
// ----------------------------- 14 -----------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64,
// }
// impl UserVault{
//     pub const LEN:usize=8+32+8+8+1
// }
// ------------------------------- 13 -----------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:usize=8+32+8+8+1
// }
// ------------------------------- 12 -----------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:usize=8+32+8+8+1
// }
// ------------------------------- 11 -----------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// ------------------------------- 10 -----------------------------

// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 9 ------------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 8 ------------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 7 ------------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 6 ------------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 5 ------------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 4 ------------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 3 ------------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 2 ------------------------------
// use anchor_lang::prelude::*;
// #[account]
// pub struct UserVault{
//     pub owner:Pubkey,
//     pub collateral:u64,
//     pub borrowed:u64,
//     pub bump:u64
// }
// impl UserVault{
//     pub const LEN:8+32+8+8+1
// }
// -------------------------------- 1 ------------------------------
use anchor_lang::prelude::*;
#[account]
pub struct UserVault{
    pub owner: Pubkey,
    pub collateral:u64,
    pub borrowed:u64,
    pub bump:u64
}
impl UserVault{
    pub const LEN:8+32+8+8+1
}