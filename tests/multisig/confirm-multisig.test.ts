import { PublicKey, Transaction } from "@solana/web3.js";
import {
  executeTransaction,
  GLOBAL_SEED,
  MockFactory,
  MULTISIG_SEED,
} from "../../sdk/common";

beforeEach(async () => {
  MockFactory.create();
});

describe.skip("Confirm Multisig", () => {
  it("should confirm the multisig account", async () => {
    const { program, backendWallet, connection, owner1, owner2 } =
      MockFactory.mockFactory;

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = new Transaction();
    const ix = await program.methods
      .confirmMultisig()
      .accounts({
        signer: backendWallet.publicKey,
        global: globalPda,
      })
      .remainingAccounts([
        {
          pubkey: backendWallet.publicKey,
          isSigner: true,
          isWritable: true,
        },
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
      ])
      .instruction();

    tx.add(ix);

    const eventListener = program.addEventListener(
      "multisigUpdatedEvent",
      (event) => {
        expect(event.threshold).toBe(3);
      }
    );

    await executeTransaction(connection, tx, backendWallet, {
      signers: [backendWallet.payer, owner1.payer, owner2.payer],
    });

    await new Promise((resolve) => setTimeout(resolve, 3000));

    program.removeEventListener(eventListener);

    const [multisigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(MULTISIG_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const multisig = await program.account.multisigState.fetch(multisigPda);

    expect(multisig.owners.length).toEqual(4);
    expect(multisig.threshold).toEqual(3);
  });
});
