import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory } from "../../sdk/common";
import { BN } from "bn.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Buy Presale Tests", () => {
  it.skip("should buy public presale", async () => {
    const { program, connection, backendWallet, usdcMint, user3 } =
      MockFactory.mockFactory;

    const usdcAmount = 500;

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const mainUsdcAta = await getOrCreateAssociatedTokenAccount(
      connection,
      backendWallet.payer,
      usdcMint,
      backendWallet.publicKey
    );

    const userUsdtAta = await getOrCreateAssociatedTokenAccount(
      connection,
      backendWallet.payer,
      usdcMint,
      user3.publicKey
    );

    const tx = new Transaction();
    const ix = await program.methods
      .buyPresale(new BN(usdcAmount * 1e6), true)
      .accounts({
        buyer: user3.publicKey,
        adminUsdcAccount: mainUsdcAta.address,
        buyerUsdcAccount: userUsdtAta.address,
        global: globalPda,
      })
      .instruction();

    tx.add(ix);

    const eventListener = program.addEventListener(
      "presalePurchaseEvent",
      (event) => {
        expect(event.buyer.toString()).toEqual(user3.publicKey.toString());
        expect(event.usdcAmount.toString()).toBe(
          new BN(usdcAmount * 1e6).toString()
        );
        expect(event.tokenAmount.toString()).toBe(
          new BN((usdcAmount / 0.0016) * 1e9).toString()
        );
      }
    );

    await new Promise((resolve) => setTimeout(resolve, 3000));

    program.removeEventListener(eventListener);

    const txHash = await executeTransaction(connection, tx, user3);
    console.log("Transaction Hash", txHash);
  });

  it("should buy private presale", async () => {
    const { program, connection, backendWallet, usdcMint, user1 } =
      MockFactory.mockFactory;

    const usdcAmount = 500;

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const mainUsdcAta = await getOrCreateAssociatedTokenAccount(
      connection,
      backendWallet.payer,
      usdcMint,
      backendWallet.publicKey
    );

    const userUsdtAta = await getOrCreateAssociatedTokenAccount(
      connection,
      backendWallet.payer,
      usdcMint,
      user1.publicKey
    );

    const tx = new Transaction();
    const ix = await program.methods
      .buyPresale(new BN(usdcAmount * 1e6), true)
      .accounts({
        buyer: user1.publicKey,
        adminUsdcAccount: mainUsdcAta.address,
        buyerUsdcAccount: userUsdtAta.address,
        global: globalPda,
      })
      .instruction();

    tx.add(ix);

    const eventListener = program.addEventListener(
      "presalePurchaseEvent",
      (event) => {
        expect(event.buyer.toString()).toEqual(user1.publicKey.toString());
        expect(event.usdcAmount.toString()).toBe(
          new BN(usdcAmount * 1e6).toString()
        );
        expect(event.tokenAmount.toString()).toBe(
          new BN((usdcAmount / 0.00112) * 1e9).toString()
        );
      }
    );

    const txHash = await executeTransaction(connection, tx, user1);
    console.log("Transaction Hash", txHash);

    await new Promise((resolve) => setTimeout(resolve, 3000));

    program.removeEventListener(eventListener);
  });
});
