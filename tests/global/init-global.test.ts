import { PublicKey, Transaction } from "@solana/web3.js";
import { executeTransaction, GLOBAL_SEED, MockFactory, MULTISIG_SEED } from "../../sdk/common";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Global Init Tests", () => {
  it("should initialize the global", async () => {
    const { program, backendWallet, treasuryWallet, tokenMint, connection } =
      MockFactory.mockFactory;

    // const [treasuryAuthorityPda] = PublicKey.findProgramAddressSync(
    //   [Buffer.from("treasury"), treasuryWallet.publicKey.toBuffer()],
    //   program.programId
    // );

    // const treasuryTokenAccount = await getOrCreateAssociatedTokenAccount(
    //   connection,
    //   treasuryWallet.payer,
    //   tokenMint,
    //   treasuryAuthorityPda,
    //   true
    // );

    const tx = new Transaction();
    const ix = await program.methods
      .initGlobal()
      .accounts({
        payer: backendWallet.publicKey,
        veraluxTokenMint: tokenMint,
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
