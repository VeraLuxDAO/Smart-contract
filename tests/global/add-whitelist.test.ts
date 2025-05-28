import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Add Whitelist Test", () => {
  it("should add new whitelist member", async () => {
    const { program, backendWallet, connection, owner1, owner2, user1 } =
      MockFactory.mockFactory;

    const [globalPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = new Transaction();
    const ix = await program.methods
      .addWhitelist(user1.publicKey)
      .accounts({ payer: backendWallet.publicKey, global: globalPda })
      .remainingAccounts([
        { pubkey: backendWallet.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner1.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner2.publicKey, isSigner: true, isWritable: true },
      ])
      .instruction();

    tx.add(ix);

    const eventListener = program.addEventListener(
      "whitelistAddedEvent",
      (event) => {
        expect(event.address.toString()).toBe(user1.publicKey.toString());
        expect(event.totalWhitelisted).toBe(1);
      }
    );

    const txHash = await executeTransaction(connection, tx, backendWallet, {
      signers: [backendWallet.payer, owner1.payer, owner2.payer],
    });
    console.log("Transaction Hash", txHash);

    await new Promise((resolve) => setTimeout(resolve, 3000));

    program.removeEventListener(eventListener);
  });
});
