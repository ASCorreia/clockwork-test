import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ClockworkTest } from "../target/types/clockwork_test";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { ClockworkProvider } from "@clockwork-xyz/sdk";

describe("clockwork-test", () => {
  // Configure the client to use the local cluster.
  const key = anchor.AnchorProvider.env();
  anchor.setProvider(key);
  const clockworkProvider = ClockworkProvider.fromAnchorProvider(key);

  const program = anchor.workspace.ClockworkTest as Program<ClockworkTest>;

  const userKeypair = Keypair.generate();
  const [dummyAccountPDA] = findProgramAddressSync([anchor.utils.bytes.utf8.encode("ClockTest")], program.programId);

  it("Airdrop 1 SOL to our user keypair", async () => {
    const airdropTx = await key.connection.requestAirdrop(userKeypair.publicKey, 1 * LAMPORTS_PER_SOL);
    let latestBlockHash = await key.connection.getLatestBlockhash();
    await key.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropTx,
    })
    console.log("\n\nAirdrop sent to user address! TxID: ", airdropTx);
  });

  it("Increments every 10 seconds", async () => {    
    // 1️⃣ Prepare thread address
    const threadId = "counter";
    const [threadAuthority] = PublicKey.findProgramAddressSync(
        // Make sure it matches on the prog side
        [anchor.utils.bytes.utf8.encode("authority")], 
        program.programId
    );
    
    const [threadAddress] = clockworkProvider.getThreadPDA(threadAuthority, threadId)
    
    // 2️⃣ Ask our program to initialize a thread via CPI
    // and thus become the admin of that thread
   const tx = await program.methods
    .initialize(Buffer.from(threadId))
    .accounts({
        payer: key.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        clockworkProgram: clockworkProvider.threadProgram.programId,
        thread: threadAddress,
        threadAuthority: threadAuthority,
        dummyAccount: dummyAccountPDA,
    })
    .rpc({skipPreflight: true}); 

    console.log("\n\nClockwork thread started! TxID: ", tx);
  });

  it("Pause thread", async() => {
    // 1️⃣ Prepare thread address
    const threadId = "counter";
    const [threadAuthority] = PublicKey.findProgramAddressSync(
        // Make sure it matches on the prog side
        [anchor.utils.bytes.utf8.encode("authority")], 
        program.programId
    );
    
    const [threadAddress] = clockworkProvider.getThreadPDA(threadAuthority, threadId);

    // 2️⃣ Ask our program to pause a thread via CPI
   let tx = await program.methods
   .pause()
   .accounts({
      systemProgram: SystemProgram.programId,
      clockworkProgram: clockworkProvider.threadProgram.programId,
      thread: threadAddress,
      threadAuthority: threadAuthority,
      dummyAccount: dummyAccountPDA,
   })
   .rpc({skipPreflight: true}); 

   console.log("\n\nClockwork thread paused! TxID: ", tx);
  });

  it("Delete thread", async() => {
    // 1️⃣ Prepare thread address
    const threadId = "counter";
    const [threadAuthority] = PublicKey.findProgramAddressSync(
        // Make sure it matches on the prog side
        [anchor.utils.bytes.utf8.encode("authority")], 
        program.programId
    );

    const [threadAddress, threadBump] = clockworkProvider.getThreadPDA(threadAuthority, threadId);

    // 2️⃣ Ask our program to delete a thread via CPI
   let tx = await program.methods
   .delete()
   .accounts({
      clockworkProgram: clockworkProvider.threadProgram.programId,
      user: key.wallet.publicKey,
      thread: threadAddress,
      threadAuthority: threadAuthority,
      dummyAccount: dummyAccountPDA,
   })
   .rpc({skipPreflight: true}); 

   console.log("\n\nClockwork thread deleted! TxID: ", tx);
  });

  it("Close Dummy Account", async() => {
    const tx = await program.methods
    .closeAccount()
    .accounts({
      destination: key.wallet.publicKey,
      dummyAccount: dummyAccountPDA,
    }).rpc();

    console.log("\n\nDummy account closed! TxID: ", tx);
  });
})
