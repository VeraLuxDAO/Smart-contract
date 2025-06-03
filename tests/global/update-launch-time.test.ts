import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";
import { BN } from "bn.js";

beforeAll(async () => {
  await MockFactory.create();
});

describe("Update Launchtime Test", () => {
  it("should update launchtime", async () => {
    const { program, backendWallet, connection, owner1, owner2, owner3 } =
      MockFactory.mockFactory;

    const [globalPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const global = await program.account.globalState.fetch(globalPda);

    console.log(global.presaleActive);

    const launchDate = new Date("2025-05-17T00:00:00");
    const launchTimestamp = new BN(Math.floor(launchDate.getTime() / 1000));

    const tx = new Transaction();
    const ix = await program.methods
      .updateLaunchTime(launchTimestamp)
      .accounts({ payer: backendWallet.publicKey, global: globalPda })
      .remainingAccounts([
        { pubkey: backendWallet.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner1.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner2.publicKey, isSigner: true, isWritable: true },
      ])
      .instruction();

    tx.add(ix);

    const eventListener = program.addEventListener(
      "updateLaunchTimeEvent",
      (event) => {
        expect(event.launchtime).toBe(launchTimestamp);
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
