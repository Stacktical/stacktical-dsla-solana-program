import { expect } from "chai";
import {
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  SLA_REGISTRY_DEPLOYER,
  SLA_REGISTRY_SPACE,
  SLA_REGISTRY_KEYPAIR,
} from "./constants";
import { connection, program } from "./init";

describe("Initialize SLA registry", () => {
  before(async function () {
    const rentExemptionAmount =
      await connection.getMinimumBalanceForRentExemption(SLA_REGISTRY_SPACE);

    const createAccountParams = {
      fromPubkey: SLA_REGISTRY_DEPLOYER.publicKey,
      newAccountPubkey: SLA_REGISTRY_KEYPAIR.publicKey,
      lamports: rentExemptionAmount,
      space: SLA_REGISTRY_SPACE,
      programId: program.programId,
    };

    let airdropSignature = await connection.requestAirdrop(
      SLA_REGISTRY_DEPLOYER.publicKey,
      LAMPORTS_PER_SOL * 1000
    );
    await connection.confirmTransaction(airdropSignature);

    const createAccountTransaction = new Transaction().add(
      SystemProgram.createAccount(createAccountParams)
    );

    await sendAndConfirmTransaction(connection, createAccountTransaction, [
      SLA_REGISTRY_DEPLOYER,
      SLA_REGISTRY_KEYPAIR,
    ]);
  });

  it("initializes an SLA registry", async () => {
    await program.methods
      .initSlaRegistry()
      .accounts({
        deployer: SLA_REGISTRY_DEPLOYER.publicKey,
        slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
      })
      .signers([SLA_REGISTRY_DEPLOYER])
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
