import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Global Init Tests", () => {
  it("should initialize the global", async () => {
    const { program, backendWallet, connection } = MockFactory.mockFactory;

    const tx = new Transaction();
    const ix = await program.methods
      .initGlobal()
      .accounts({
        payer: backendWallet.publicKey,
      })
      .instruction();

    tx.add(ix);
    await executeTransaction(connection, tx, backendWallet);

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const global = await program.account.globalState.fetch(globalPda);

    expect(global.paused).toEqual(true);
  });
});
