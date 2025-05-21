import { PublicKey, Transaction } from "@solana/web3.js";
import {
  executeTransaction,
  GLOBAL_SEED,
  MockFactory,
  MULTISIG_SEED,
} from "../../sdk/common";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { BN } from "bn.js";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Global Update Tests", () => {
  it("should update the treasury wallet", async () => {
    const { program, backendWallet, connection, owner1, owner2 } =
      MockFactory.mockFactory;

    // const treasuryUsdtAta = getAssociatedTokenAddressSync(
    //   usdtMint,
    //   treasuryWallet.publicKey
    // );

    const launchDate = new Date("2025-05-22T01:00:00");
    const launchTimestamp = new BN(Math.floor(launchDate.getTime() / 1000));

    const tx = new Transaction();
    const ix = await program.methods
      .updateGlobal({
        initialOwners: [
          backendWallet.publicKey,
          owner1.publicKey,
          owner2.publicKey,
        ],
        launchTimestamp,
        threshold: 2,
      })
      .accounts({
        payer: backendWallet.publicKey,
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
          backendWallet.publicKey.toBase58(),
          owner1.publicKey.toBase58(),
          owner2.publicKey.toBase58(),
        ]);
        expect(event.threshold).toBe(2);
      }
    );

    await executeTransaction(connection, tx, backendWallet);

    await new Promise((resolve) => setTimeout(resolve, 500));

    program.removeEventListener(eventListener);

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const [multisigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(MULTISIG_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const global = await program.account.globalState.fetch(globalPda);

    expect(global.launchTimestamp.eq(launchTimestamp)).toBe(true);
    expect(global.admin.equals(multisigPda)).toBe(true);
  });
});
