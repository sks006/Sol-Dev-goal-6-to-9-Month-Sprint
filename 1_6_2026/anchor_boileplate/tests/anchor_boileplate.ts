// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { AnchorBoileplate } from "../target/types/anchor_boileplate";
// import { expect } from "chai";
// describe("anchor_boileplate", () => {
//   // Configure the client to use the local cluster.
//    const provider = anchor.AnchorProvider.env();
//    anchor.setProvider(provider);

//   const program = anchor.workspace.anchorBoileplate as Program<AnchorBoileplate>;

//   it("Is initialized!", async () => {
//     //Add your test here.
//         const amount = new anchor.BN(5000000);

//     // Find the PDA (Mental Map of Solana)
//     const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("vault"), provider.wallet.publicKey.toBuffer()],
//       program.programId
//     );

//     // ACTION: The Handshake
//     await program.methods
//       .deposit(amount) // If this shows red, run 'anchor build' first
//       .accounts({
//         vaultAccount: vaultPDA,
//         user: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .rpc();

//     // VERIFICATION
//     const vaultData = await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString()).to.equal(amount.toString());

//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });
//-------------------------------------------- 6 ------------------------------

// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { AnchorBoileplate } from "../target/types/anchor_boileplate";
// import { expect } from "chai";
// import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
// describe("anchor_boileplate", () => {
//   const provider =anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program =anchor.workspace.anchorBoileplate as Program<AnchorBoileplate>;
//   it("Lending",async()=>{
//     const amount=new anchor.BN(5000000);
//     const [vaultPDA]= anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
//       program.programId
//     )
//     await program.methods.deposit(amount).accounts({
//       vaultAccount:vaultPDA,
//       user:provider.wallet.publicKey,
//       systemProgram:anchor.web3.SystemProgram.programId,
//     }).rpc()
//     const vaultData=await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString().to.equal(amount.toString()));
//     const tx=await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   })
// })
// -------------------------------------------- 5 ------------------------------
// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { AnchorBoileplate } from "../target/types/anchor_boileplate";
// import { expect } from "chai";
// describe("anchor_boileplate", () => {
//   const provider=anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program=anchor.workspace.anchorBoileplate as Program<AnchorBoileplate>;
//   it("Lending",async()=>{
//     const amount = new anchor.BN(5000000);
//     const [vaultPDA]=anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
//       program.programId
//     )
//     await program.methods.deposit(amount)
//     .accounts({
//       vaultAccount:vaultPDA,
//       user:provider.wallet.publicKey,
//       systemProgram:anchor.web3.SystemProgram.programId,
//     }).rpc();
//     const vaultData=await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString()).to.equal(amount.toString());
//     const tx=await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx)
//   })
// })
// -------------------------------------------- 5 ------------------------------
// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { AnchorBoileplate } from "../target/types/anchor_boileplate";
// import { expect } from "chai";
// describe("anchor_boileplate", () => {
//   const provider=anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program=anchor.workspace.anchorBoileplate as Program<AnchorBoileplate>;

//   it("lending", async()=>{
//     const amount=new anchor.BN(5000000);
//     const [vaultPDA]=anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
//       program.programId
//     )
//     await program.methods.deposit(amount).accounts({
//       vaultAccount:vaultPDA,
//       user:provider.wallet.publicKey,
//       systemProgram:anchor.web3.SystemProgram.programId
//     }).rpc()
//         // VERIFICATION
//     const vaultData = await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString()).to.equal(amount.toString());

//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   })
// })
// -------------------------------------------- 4 ------------------------------
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorBoileplate } from "../target/types/anchor_boileplate";
import { expect } from "chai";
describe("anchor_boileplate", () => {
  // Configure the client to use the local cluster.
   const provider = anchor.AnchorProvider.env();
   anchor.setProvider(provider);

  const program = anchor.workspace.anchorBoileplate as Program<AnchorBoileplate>;

  it("Is initialized!", async () => {
    //Add your test here.
        const amount = new anchor.BN(5000000);

    // Find the PDA (Mental Map of Solana)
    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // ACTION: The Handshake
    await program.methods
      .deposit(amount) // If this shows red, run 'anchor build' first
      .accounts({
        vaultAccount: vaultPDA,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // VERIFICATION
    const vaultData = await program.account.userVault.fetch(vaultPDA);
    expect(vaultData.collateral.toString()).to.equal(amount.toString());

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});