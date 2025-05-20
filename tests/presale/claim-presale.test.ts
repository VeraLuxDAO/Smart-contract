import { PublicKey, Transaction } from "@solana/web3.js";
import {
  executeTransaction,
  GLOBAL_SEED,
  MockFactory,
  TREASURY_SEED,
} from "../../sdk/common";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Claim Presale Tests", () => {
  it("should claim presale", async () => {
    const { program, userWallet, connection, treasuryWallet, tokenMint } =
      MockFactory.mockFactory;

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED)],
      program.programId
    );

    const [treasuryAuthorityPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(TREASURY_SEED), treasuryWallet.publicKey.toBuffer()],
      program.programId
    );

    const treasuryTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      treasuryWallet.payer,
      tokenMint,
      treasuryAuthorityPda,
      true
    );

    const userTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      userWallet.payer,
      tokenMint,
      userWallet.publicKey
    );

    const tx = new Transaction();
    const ix = await program.methods
      .claimPresale()
      .accounts({
        global: globalPda,
        user: userWallet.publicKey,
        treasuryTokenAccount: treasuryTokenAccount.address,
        userTokenAccount: userTokenAccount.address,
      })
      .instruction();

    tx.add(ix);
    await executeTransaction(connection, tx, userWallet);

    const userTokenAccountBalance = await connection.getTokenAccountBalance(
      userTokenAccount.address
    );

    console.log(userTokenAccountBalance);
  });
});
