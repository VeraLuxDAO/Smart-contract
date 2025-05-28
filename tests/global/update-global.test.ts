import { PublicKey, Transaction } from "@solana/web3.js";
import {
  executeTransaction,
  GLOBAL_SEED,
  MockFactory,
  MULTISIG_SEED,
} from "../../sdk/common";
import { BN } from "bn.js";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Global Update Tests", () => {
  it("should update the glbal infos", async () => {
    const {
      program,
      backendWallet,
      connection,
      owner1,
      owner2,
      charityWallet,
      treasuryWallet,
      teamWallet,
      LPWallet,
    } = MockFactory.mockFactory;

    const launchDate = new Date("2025-05-27T00:00:00");
    const launchTimestamp = new BN(Math.floor(launchDate.getTime() / 1000));

    const [globalPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = new Transaction();
    const ix = await program.methods
      .updateGlobal({
        charityWallet: charityWallet.publicKey,
        lpWallet: LPWallet.publicKey,
        teamWallet: teamWallet.publicKey,
        treasuryWallet: treasuryWallet.publicKey,
        threshold: 2,
        initialOwners: [
          backendWallet.publicKey,
          owner1.publicKey,
          owner2.publicKey,
        ],
        launchTimestamp,
      })
      .accounts({
        payer: backendWallet.publicKey,
        global: globalPda,
      })
      .instruction();

    tx.add(ix);

    const eventListener = program.addEventListener(
      "globalUpdatedEvent",
      (event) => {
        expect(event.launchTimestamp.toString()).toBe(
          launchTimestamp.toString()
        );
        expect(event.initialOwners).toEqual([
          backendWallet.publicKey,
          owner1.publicKey,
          owner2.publicKey,
        ]);
        expect(event.threshold).toBe(2);
      }
    );

    const txHash = await executeTransaction(connection, tx, backendWallet);
    console.log("Transaction Hash", txHash);

    await new Promise((resolve) => setTimeout(resolve, 500));

    program.removeEventListener(eventListener);

    const [multisigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(MULTISIG_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const global = await program.account.globalState.fetch(globalPda);

    expect(global.launchTimestamp.eq(launchTimestamp)).toBe(true);
    expect(global.admin.equals(multisigPda)).toBe(true);
    expect(global.paused).toBe(false);
  });
});
