import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import {
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import {
  DEPLOYER,
  GOVERNANCE_SEED,
  SLA_REGISTRY_SPACE,
  GOVERNANCE_PARAMETERS,
  SLA_REGISTRY_KEYPAIR,
} from "./constants";

describe("Initialize SLA registry", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;

  before(async function () {
    const rentExemptionAmount =
      await connection.getMinimumBalanceForRentExemption(SLA_REGISTRY_SPACE);

    const createAccountParams = {
      fromPubkey: DEPLOYER.publicKey,
      newAccountPubkey: SLA_REGISTRY_KEYPAIR.publicKey,
      lamports: rentExemptionAmount,
      space: SLA_REGISTRY_SPACE,
      programId: program.programId,
    };

    let airdropSignature = await connection.requestAirdrop(
      DEPLOYER.publicKey,
      LAMPORTS_PER_SOL * 1000
    );
    await connection.confirmTransaction(airdropSignature);

    const createAccountTransaction = new Transaction().add(
      SystemProgram.createAccount(createAccountParams)
    );

    await sendAndConfirmTransaction(connection, createAccountTransaction, [
      DEPLOYER,
      SLA_REGISTRY_KEYPAIR,
    ]);
  });

  it("initializes an SLA registry", async () => {
    const [governancePda, _governanceBump] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode(GOVERNANCE_SEED)],
      program.programId
    );

    await program.methods
      .initSlaRegistry(GOVERNANCE_PARAMETERS)
      .accounts({
        deployer: DEPLOYER.publicKey,
        governance: governancePda,
        slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
      })
      .signers([DEPLOYER])
      .rpc();

    const expectedSlaAccountAddresses = [];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(SLA_REGISTRY_KEYPAIR.publicKey)
    ).slaAccountAddresses;

    expect(
      actualSlaAccountAddresses,
      "SLA registry address doesn't match  the expected address"
    ).to.deep.equal(expectedSlaAccountAddresses);

    expect(
      actualSlaAccountAddresses.length,
      "SLA registry lenghth doesn't match"
    ).to.equal(expectedSlaAccountAddresses.length);
  });
});
