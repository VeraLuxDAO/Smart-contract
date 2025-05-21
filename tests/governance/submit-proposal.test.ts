import { PublicKey, Transaction } from "@solana/web3.js";
import {
  executeTransaction,
  GLOBAL_SEED,
  MockFactory,
  PROPOSAL_SEED,
} from "../../sdk/common";
import { BN } from "bn.js";
import { ProposalType } from "../../tools/types";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Submit proposal test", () => {
  it("should submit the new proposal", async () => {
    const { program, connection, backendWallet, owner1, owner2, owner3 } =
      MockFactory.mockFactory;

    const [globalPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from(GLOBAL_SEED), backendWallet.publicKey.toBuffer()],
      program.programId
    );

    const global = await program.account.globalState.fetch(globalPda);

    const testDescription = "Test Description - 2";
    const proposalType = ProposalType["UpdateTaxRate"];

    const descriptionBuffer = Buffer.from(testDescription, "utf-8");

    const proposalValues: any[] = [];
    const proposalValuesBN = proposalValues.map((value) => new BN(value));

    const tx = new Transaction();
    const ix = await program.methods
      .submitProposal({
        description: descriptionBuffer,
        proposalType: proposalType,
        proposalValues: proposalValuesBN,
      })
      .accounts({
        signer: backendWallet.publicKey,
        global: globalPda,
      })
      .remainingAccounts([
        { pubkey: backendWallet.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner1.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner2.publicKey, isSigner: true, isWritable: true },
        { pubkey: owner3.publicKey, isSigner: true, isWritable: true },
      ])
      .instruction();

    tx.add(ix);

    const eventListener = program.addEventListener(
      "proposalSubmittedEvent",
      (event) => {
        expect(event.proposalId).toBe(global.proposalCount);
        expect(event.description).toEqual(testDescription);
        expect(event.proposalType).toBe(proposalType);
      }
    );

    await executeTransaction(connection, tx, backendWallet, {
      signers: [backendWallet.payer, owner1.payer, owner2.payer, owner3.payer],
    });

    await new Promise((resolve) => setTimeout(resolve, 3000));

    program.removeEventListener(eventListener);

    const bufferLE = Buffer.alloc(4);
    bufferLE.writeUint32LE(global.proposalCount, 0);

    const [proposalPda] = await PublicKey.findProgramAddressSync(
      [
        Buffer.from(PROPOSAL_SEED),
        backendWallet.publicKey.toBuffer(),
        bufferLE,
      ],
      program.programId
    );

    const proposal = await program.account.proposalState.fetch(proposalPda);

    expect(proposal.id).toBe(global.proposalCount);
    expect(proposal.proposalType).toBe(proposalType);
    expect(proposal.description.toString()).toEqual(testDescription);
  });
});
