import { PublicKey, Transaction } from "@solana/web3.js";
import {
  executeTransaction,
  GLOBAL_SEED,
  MockFactory,
  PENDING_MULTISIG_SEED,
} from "../../sdk/common";

beforeEach(async () => {
  MockFactory.create();
});

describe.skip("Init Multisig", () => {
  it("should initialize a multisig account", async () => {
    const { program, backendWallet, connection, owner1, owner2, owner3 } =
      MockFactory.mockFactory;

    const [globalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = new Transaction();
    const ix = await program.methods
      .initMultisig(
        [
          backendWallet.publicKey,
          owner1.publicKey,
          owner2.publicKey,
          owner3.publicKey,
        ],
        3
      )
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
      "multisigPendingEvent",
      (event) => {
        expect(event.threshold).toBe(3);
      }
    );

    await executeTransaction(connection, tx, backendWallet, {
      signers: [backendWallet.payer, owner1.payer, owner2.payer],
    });

    await new Promise((resolve) => setTimeout(resolve, 3000));

    program.removeEventListener(eventListener);

    const [pendingMultisigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(PENDING_MULTISIG_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const pendingMultisig = await program.account.pendingMultisigState.fetch(
      pendingMultisigPda
    );
    expect(pendingMultisig.newThreshold).toBe(3);
  });
});
