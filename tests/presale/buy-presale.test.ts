import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";
import { BN } from "bn.js";
import {
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Buy Presale Tests", () => {
  it("should buy presale", async () => {
    const {
      program,
      userWallet,
      connection,
      usdtMint,
      treasuryWallet,
      backendWallet,
    } = MockFactory.mockFactory;

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED)],
      program.programId
    );

    const treasuryUsdtAta = await getOrCreateAssociatedTokenAccount(
      connection,
      backendWallet.payer,
      usdtMint,
      treasuryWallet.publicKey
    );

    const userUsdtAta = getAssociatedTokenAddressSync(
      usdtMint,
      userWallet.publicKey
    );

    const tx = new Transaction();
    const ix = await program.methods
      .buyPresale(new BN(1000), true)
      .accounts({
        purchaser: userWallet.publicKey,
        global: globalPda,
        purchaserUsdt: userUsdtAta,
        treasuryUsdt: treasuryUsdtAta.address,
      })
      .instruction();

    tx.add(ix);
    await executeTransaction(connection, tx, userWallet);

    const global = await program.account.globalState.fetch(globalPda);
    const treasuryAmount = (
      await getAccount(connection, global.treasuryUsdtAccount)
    ).amount.toString();
    expect(treasuryAmount).toBe("1000");
  });
});
