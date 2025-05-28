import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Start Presale Test", () => {
  it("should start presale", async () => {
    const { program, backendWallet, connection, owner1, owner2, owner3 } =
      MockFactory.mockFactory;

    const [globalPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = new Transaction();
    const ix = await program.methods
      .startPresale()
      .accounts({ payer: backendWallet.publicKey, global: globalPda })
      .remainingAccounts([
        { pubkey: backendWallet.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner1.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner2.publicKey, isSigner: true, isWritable: true },
      ])
      .instruction();

    tx.add(ix);

    const eventListener = program.addEventListener(
      "startedPresaleEvent",
      (event) => {
        expect(event.startedPresale).toBe(true);
      }
    );

    const txHash = await executeTransaction(connection, tx, backendWallet, {
      signers: [backendWallet.payer, owner1.payer, owner2.payer],
    });
    console.log("Transaction Hash", txHash);

    await new Promise((resolve) => setTimeout(resolve, 3000));

    program.removeEventListener(eventListener);

    const glbal = await program.account.globalState.fetch(globalPda);

    expect(glbal.presaleActive).toBe(true);
  });
});
