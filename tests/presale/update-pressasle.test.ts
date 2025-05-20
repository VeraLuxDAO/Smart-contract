import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";
import { BN } from "bn.js";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Update Presale Tests", () => {
  it("should update the presale", async () => {
    const { program, backendWallet, connection } = MockFactory.mockFactory;

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED)],
      program.programId
    );

    const launchDate = new Date("2025-03-20T01:00:00");
    const launchTimestamp = Math.floor(launchDate.getTime() / 1000);

    console.log(launchDate, launchTimestamp);

    const tx = new Transaction();
    const ix = await program.methods
      .updatePresale(null, null, null, new BN(launchTimestamp), null, null)
      .accounts({ payer: backendWallet.publicKey, global: globalPda })
      .instruction();

    tx.add(ix);
    await executeTransaction(connection, tx, backendWallet);

    const global = await program.account.globalState.fetch(globalPda);

    expect(global.launchTimestamp.eq(new BN(launchTimestamp))).toBe(true);
  });
});
