import { PublicKey, Transaction } from "@solana/web3.js";
import {
  executeTransaction,
  MockFactory,
  MULTISIG_SEED,
} from "../../sdk/common";

beforeEach(async () => {
  MockFactory.create();
});

describe.skip("Init Multisig", () => {
  it("should initialize a multisig account", async () => {
    const { program, backendWallet, connection, owner1, owner2, owner3 } =
      MockFactory.mockFactory;

    const tx = new Transaction();
    const ix = await program.methods
      .initMultisig([owner1.publicKey, owner2.publicKey, owner3.publicKey], 2)
      .accounts({
        payer: backendWallet.publicKey,
      })
      .instruction();

    tx.add(ix);
    await executeTransaction(connection, tx, backendWallet);

    const [multisigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(MULTISIG_SEED)],
      program.programId
    );

    const multisig = await program.account.multisigState.fetch(multisigPda);

    expect(multisig.owners.length).toEqual(3);
    expect(multisig.owners).toEqual([
      owner1.publicKey,
      owner2.publicKey,
      owner3.publicKey,
    ]);
    expect(multisig.threshold).toEqual(2);
  });
});
