/*
AccountInfo<'info>
When to Use UncheckedAccount
While safer alternatives like SystemAccount are generally preferred,
UncheckedAccount is necessary in specific 2026 development scenarios:-----------------------
Undefined Structures: When working with accounts that lack a clearly defined data structure or Anchor type definition.

Custom Validation: If you need to implement highly specific validation logic that Anchor’s built-in types cannot handle.

Cross-Program Interaction: When interacting with external 
programs that don't provide Anchor-compatible crates, or when avoiding circular dependencies in complex CPI calls.

Performance: For extremely simple operations like basic SOL transfers where deep account verification is redundant (though SystemAccount is still the safer choice even here). 

*/

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    /// CHECK: AccountInfo is an unchecked account

    pub unchecked_account: AccountInfo<'info>,
}

// AccountLoader<'info, T>
/*
Key Characteristics
Zero-Copy: 
Instead of deserializing data into a new struct (which can hit BPF stack/heap limits), AccountLoader re-interprets the raw bytes of the account's backing RefCell<&mut [u8]> as a reference to your data structure.

Borrow Checking at Runtime: 
Just like standard Rust RefCell usage, these types track borrows at runtime. Calling load_mut() while a Ref (from load()) is still in scope will result in a RefCell panic.

Automatic Cleanup:
 These smart pointers implement the Drop trait. When a Ref or RefMut goes out of scope, it automatically updates the borrow count, allowing the account to be borrowed again (e.g., for a Cross-Program Invocation). 
Common Pitfalls

CPI Boundaries: 
You must ensure all Ref or RefMut guards are dropped before making a Cross-Program Invocation (CPI) involving that account, or the program will panic when attempting to access the data again.

Memory Layout: 
Because this pattern casts raw memory directly to a struct, your struct must use #[account(zero_copy)] and typically #[repr(C)] or #[repr(packed)] to ensure consistent data alignment between the program and the account storage

*/
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub account: AccountLoader<'info, ZeroCopyAccountType>,
}

#[account(zero_copy)]
pub struct ZeroCopyAccountType {
    data: u64,
}

// Box<Account<'info, T>>
/*
The stack has a very limited size (e.g., only 4KB per stack frame on Solana). If you define large variables or deep recursive structures directly on the stack, you may encounter a "stack overflow" error. 
Fixed Size on Stack:
A Box<T> takes up only a few bytes on the stack—the size of a single pointer—regardless of how large the data T is.
Data on Heap:
The actual data resides on the heap, which is much larger and more flexible than the stack.
Anchor Framework Use Case:
In Solana development, you can use Box<Account<'info, MyAccount>> within your #[Accounts] struct to prevent exceeding the 4KB stack limit during account deserialization. 

*/

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub account: Box<Account<'info, AccountType>>,
}
//Interface<'info, T>
/*
Purpose:
It allows an instruction to accept any program that adheres to a specific interface rather than a single, hardcoded program ID.

Common Use Case:
The most frequent use for this type is handling both the SPL Token and SPL Token 2022 programs in a single instruction.

Validation Logic:
When using this type, Anchor automatically checks if the provided program ID matches any of the IDs defined in the specified interface T
*/
// Token program or Token2022 program
use anchor_spl::token_interface::TokenInterface;

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub program: Interface<'info, TokenInterface>,
}

// InterfaceAccount<'info, T>
/*
InterfaceAccount<'info, T>:
Used for data accounts rather than program accounts. It validates that the account's owner is one of the programs in the specified set.
*/

// Token program or Token2022 program Mint/TokenAccount
use anchor_spl::token_interface::{ Mint, TokenAccount, TokenInterface };

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub mint: InterfaceAccount<'info, Mint>,
    pub token: InterfaceAccount<'info, TokenAccount>,
    pub program: Interface<'info, TokenInterface>,
}

//Option<Account<'info, T>>
/*
Core Functionality

Presence Validation: 
If an account is provided, Anchor performs standard validation (deserialization, ownership checks, and any custom constraints) and returns Some(account).

Absence Handling: 
If the account is not provided, Anchor returns None, allowing the instruction to proceed without error.

Dummy Placeholder: 
On the client side, if an optional account is omitted, Anchor typically passes the Program ID as a dummy placeholder address. The program recognizes this specific address (or a null equivalent) to treat the account as None. 
*/
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub account: Option<Account<'info, AccountType>>,
}

//Program<'info, T>
/*
Key Features:
Validation:
 It automatically verifies that the provided account is executable and that its address matches the expected program ID defined in type T.

Usage:
 It is primarily used when your instruction needs to perform Cross-Program Invocations (CPI), as it ensures you are interacting with the correct program.

Common Examples:
Program<'info, System>: Validates the account is the Solana System Program.
Program<'info, Token>: Validates the account is the SPL Token Program. 

*/
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

//Signer<'info>
/*
Key Functions

Signature Verification: 
It automatically checks the transaction's signature to ensure it matches the public key of the provided account.

Minimal Validation: 
Unlike the Account type, Signer does not perform data deserialization or ownership checks; it purely validates authorization via digital signature.

Flexible Metadata: 
While primarily used for signature checks, Signer is a wrapper around AccountInfo, allowing you to access the account's public key (.key()) or lamport balance if needed. 

*/

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub signer: Signer<'info>,
}

//SystemAccount<'info>
/*
Key Functions

Ownership Validation:
It automatically performs a check to ensure that account.owner == SystemProgram.

No Deserialization:
Unlike the generic Account<'info, T> type, it does not attempt to deserialize any data from the account. This makes it ideal for accounts that do not hold structured state, such as standard user wallets (EOAs).

Safety Over AccountInfo:
It is preferred over the raw AccountInfo or UncheckedAccount types because it provides a specific security check (ownership) rather than no check at all
*/
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub account: SystemAccount<'info>,
}
//Sysvar<'info, T>
/*
Key Features
Validation:
It verifies that the account's public key matches the well-known address for the specific sysvar T (e.g., Rent, Clock, or StakeHistory).

Automatic Deserialization:
Unlike using raw AccountInfo, this type automatically parses the sysvar data into the corresponding Rust struct T so you can access its fields directly in your instruction handler.

Safety:
It ensures that your program is reading authentic network state data maintained by the Solana runtime. 

*/
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}
//UncheckedAccount<'info>
/*

Key Features
No Validation: 
Anchor does not check ownership, data structure, or executable status for this type.

Safety Requirement:
Because it bypasses automatic security, Anchor requires a /// CHECK: comment above the field to explicitly document why the account is safe to use in an unchecked state.

Direct Access:
It provides raw access to the account's underlying data, lamports, and public key. 

🔶 When to Use It 
While it should generally be avoided in favor of more specific types like Account<'info, T> or SystemAccount<'info>, it is necessary in certain scenarios:

Arbitrary Addresses:
When you need to pass an address that may not exist yet or doesn't have a specific data structure.
Custom Validation:
When you want to manually perform your own security checks within the instruction handler rather than letting Anchor do it.

Circular Dependencies:
When interacting with programs that don't have Anchor type definitions or when crate imports would cause circular cycles.

External Program Interaction:
Representing metadata accounts created by other programs (e.g., Metaplex) where structure validation is handled manually

*/


//Migration<'info, From, To>
/*
Core Functionality
Deserialization:
When the instruction begins, the account is validated and deserialized using the older From format.

Transformation:
The developer defines the logic to map data from the From struct to the new To struct within the instruction handler.

Serialization:
Upon the instruction's completion, the account is automatically re-serialized using the new To format. 

🔶 Key Characteristics
Ownership Check:
Validates that the account is initialized and owned by the current program.

Resizing:
It is typically paired with the realloc constraint. Since a new schema often requires a different amount of space, realloc ensures the account's allocated memory is adjusted to accommodate the To format before serialization.

Efficiency: This type allows for "lazy migration," where accounts are upgraded only when they are next used, avoiding the need for expensive global migration scripts. 

*/
use anchor_lang::prelude::*;
 
#[account]
pub struct AccountV1 {
    pub data: u64,
}
 
#[account]
pub struct AccountV2 {
    pub data: u64,
    pub new_field: u64,
}
 
#[derive(Accounts)]
pub struct MigrateAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,


    #[account(
        mut,
        realloc = 8 + AccountV2::INIT_SPACE,
        realloc::payer = payer,
        realloc::zero = false
    )]
    pub my_account: Migration<'info, AccountV1, AccountV2>,
    pub system_program: Program<'info, System>,
}
fn main() {
    println!("Hello, world!");
}
