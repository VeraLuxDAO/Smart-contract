import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Global Update Tests", () => {
  it("should update the treasury wallet", async () => {
    const { program, backendWallet, connection, treasuryWallet, usdtMint } =
      MockFactory.mockFactory;

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED)],
      program.programId
    );

    const treasuryUsdtAta = getAssociatedTokenAddressSync(
      usdtMint,
      treasuryWallet.publicKey
    );

    const tx = new Transaction();
    const ix = await program.methods
      .updateGlobal(treasuryUsdtAta, null)
      .accounts({
        payer: backendWallet.publicKey,
        global: globalPda,
      })
      .instruction();

    tx.add(ix);
    await executeTransaction(connection, tx, backendWallet);

    const global = await program.account.globalState.fetch(globalPda);

    expect(global.treasuryUsdtAccount).toEqual(treasuryUsdtAta);
    expect(global.treasuryWallet).toEqual(treasuryWallet.publicKey);
  });
});
