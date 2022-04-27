import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla, IDL } from "../target/types/dsla";
import {
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

describe("Initialize SLA registry", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;
  const slaRegistryKeypair = anchor.web3.Keypair.generate();

  const deployer = Keypair.generate();

  const space = 10_000_000;

  before(async function () {
    const rentExemptionAmount =
      await connection.getMinimumBalanceForRentExemption(space);

    const createAccountParams = {
      fromPubkey: deployer.publicKey,
      newAccountPubkey: slaRegistryKeypair.publicKey,
      lamports: rentExemptionAmount,
      space,
      programId: program.programId,
    };

    let airdropSignature = await connection.requestAirdrop(
      deployer.publicKey,
      LAMPORTS_PER_SOL * 1000
    );
    await connection.confirmTransaction(airdropSignature);

    const createAccountTransaction = new Transaction().add(
      SystemProgram.createAccount(createAccountParams)
    );

    await sendAndConfirmTransaction(connection, createAccountTransaction, [
      deployer,
      slaRegistryKeypair,
    ]);
  });

  it("initializes an SLA registry", async () => {
    await program.methods
      .initSlaRegistry()
      .accounts({
        deployer: deployer.publicKey,
        slaRegistry: slaRegistryKeypair.publicKey,
      })
      .signers([deployer])
      .rpc();

    const expectedSlaAccountAddresses = [];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(slaRegistryKeypair.publicKey)
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
