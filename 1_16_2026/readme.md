## Account<'info, T>
Description: Account container that checks ownership on deserialization
1. 🦀 Ownership Validation.
The most critical check is Ownership. Every account on Solana is "owned" by a specific program [1, 5]. 
What it means: Anchor verifies that the owner field of the account matches the Program ID of your current program [4, 5].
Why it matters: This prevents a malicious user from passing in an account owned by a different program (or a System Account) that might have the same data structure but different security implications [1, 4]. 
2. 🦀 Data Decoding.
Once ownership is verified, Anchor attempts to map the raw bytes into your defined type T.
Discriminator Check: Anchor prepends an 8-byte "discriminator" (a unique hash) to the account data [2, 5]. During deserialization, Anchor checks this hash to ensure the account you are passing is actually the specific type (T) you expected, and not just any account owned by your program [2, 4]. 

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub account: Account<'info, CustomAccountType>,
}
 
#[account]
pub struct CustomAccountType {
    data: u64,
}

```
### This code provide 3 things
Discriminator Creation:
     It generates a unique 8-byte "discriminator" (based on the SHA256 hash of the struct's name). This identifier is stored at the very beginning of the account's data on-chain.
Serialization/Deserialization:
     It implements the logic needed to convert your Rust struct into raw bytes (to save to the blockchain) and back again (to read from it).
Ownership Assignment: 
     It ensures that when this account is initialized, it is owned by the program you are currently writing.