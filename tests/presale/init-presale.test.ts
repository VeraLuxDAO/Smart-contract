import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";
import { BN } from "bn.js";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Init Presale Tests", () => {
  it("should initialize the presale", async () => {
    const { program, backendWallet, connection } = MockFactory.mockFactory;

    const launchDate = new Date(2025, 5, 20, 2, 0, 0);
    const launchTimestamp = Math.floor(launchDate.getTime() / 1000);

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED)],
      program.programId
    );

    const tx = new Transaction();
    const ix = await program.methods
      .initPresale(
        new BN(1600),
        new BN(2000000),
        new BN(250000000),
        new BN(launchTimestamp),
        {
          initialUnlockPct: 10,
          weeklyUnlockPct: 10,
        }
      )
      .accounts({
        payer: backendWallet.publicKey,
        global: globalPda,
      })
      .instruction();

    tx.add(ix);
    await executeTransaction(connection, tx, backendWallet);

    const global = await program.account.globalState.fetch(globalPda);

    expect(global.tokenPriceInUsdt.eq(new BN(1600))).toBe(true);
    expect(global.maxPerWallet.eq(new BN(2000000))).toBe(true);
    expect(global.totalPresaleCap.eq(new BN(250000000))).toBe(true);
    expect(global.launchTimestamp.eq(new BN(launchTimestamp))).toBe(true);
    expect(global.presaleActive).toBe(true);
  });
});
