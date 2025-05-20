import { PublicKey, Transaction } from "@solana/web3.js";
import {
  executeTransaction,
  MockFactory,
  MULTISIG_SEED,
} from "../../sdk/common";

beforeEach(async () => {
  MockFactory.create();
});

describe.skip("Update Multisig", () => {
  it("should update the multisig account", async () => {
    const {
      program,
      backendWallet,
      connection,
      owner1,
      owner2,
      owner3,
      owner4,
    } = MockFactory.mockFactory;

    const tx = new Transaction();
    const ix = await program.methods
      .updateMultisig([owner1.publicKey, owner4.publicKey], 2)
      .accounts({
        payer: backendWallet.publicKey,
      })
      .remainingAccounts([
        {
          pubkey: owner1.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: owner2.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: owner3.publicKey,
          isSigner: true,
          isWritable: true,
        },
      ])
      .instruction();

    tx.add(ix);
    await executeTransaction(connection, tx, backendWallet, {
      signers: [backendWallet.payer, owner1.payer, owner2.payer, owner3.payer],
    });

    const [multisigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(MULTISIG_SEED)],
      program.programId
    );

    const multisig = await program.account.multisigState.fetch(multisigPda);

    expect(multisig.owners.length).toEqual(2);
    expect(multisig.owners).toEqual([owner1.publicKey, owner4.publicKey]);
  });
});
