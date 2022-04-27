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
  PublicKey,
} from "@solana/web3.js";

describe("Deploy SLA", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;
  const slaRegistryKeypair = anchor.web3.Keypair.generate();

  const deployer = Keypair.generate();

  const space = 10_000_000;
  const slaKeypairs = [
    Keypair.generate(),
    Keypair.generate(),
    Keypair.generate(),
  ];
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

    await program.methods
      .initSlaRegistry()
      .accounts({
        deployer: deployer.publicKey,
        slaRegistry: slaRegistryKeypair.publicKey,
      })
      .signers([deployer])
      .rpc();
  });

  it("Deploys an SLA 1", async () => {
    const ipfsHash = "t";
    let sloType = { greaterThan: {} };
    const slo = { sloValue: new anchor.BN("100"), sloType };
    const messengerAddress = anchor.web3.Keypair.generate().publicKey;
    const periods = [
      {
        start: new anchor.BN("1000000"),
        end: new anchor.BN("1900000"),
        status: { notVerified: {} },
      },
    ];
    const leverage = new anchor.BN("1");

    const [periodRegistryPda, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("period-registry"),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .deploySla(ipfsHash, slo, messengerAddress, periods, leverage)
      .accounts({
        deployer: deployer.publicKey,
        slaRegistry: slaRegistryKeypair.publicKey,
        sla: slaKeypairs[0].publicKey,
        periodRegistry: periodRegistryPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([deployer, slaKeypairs[0]])
      .rpc();

    const expectedSlaAccountAddresses = [slaKeypairs[0].publicKey];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(slaRegistryKeypair.publicKey)
    ).slaAccountAddresses;

    expect(
      actualSlaAccountAddresses[0].toString(),
      "SLA registry address doesn't match  the expected address"
    ).to.equal(expectedSlaAccountAddresses[0].toString());

    expect(
      actualSlaAccountAddresses.length,
      "SLA registry lenghth doesn't match"
    ).to.equal(expectedSlaAccountAddresses.length);

    expect(
      actualSlaAccountAddresses[0].toString(),
      "match to wrong address"
    ).to.not.equal(slaKeypairs[1].publicKey.toString());
  });

  it("Deploys an SLA 2", async () => {
    const ipfsHash = "tt";
    let sloType = { smallerThan: {} };
    const slo = { sloValue: new anchor.BN("999"), sloType };
    const messengerAddress = anchor.web3.Keypair.generate().publicKey;
    const periods = [
      {
        start: new anchor.BN("99999999"),
        end: new anchor.BN("10000000000"),
        status: { notVerified: {} },
      },
    ];
    const leverage = new anchor.BN("5");

    const [periodRegistryPda, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("period-registry"),
        slaKeypairs[1].publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .deploySla(ipfsHash, slo, messengerAddress, periods, leverage)
      .accounts({
        deployer: deployer.publicKey,
        slaRegistry: slaRegistryKeypair.publicKey,
        sla: slaKeypairs[1].publicKey,
        periodRegistry: periodRegistryPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([deployer, slaKeypairs[1]])
      .rpc();

    const expectedSlaAccountAddresses = [
      slaKeypairs[0].publicKey,
      slaKeypairs[1].publicKey,
    ];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(slaRegistryKeypair.publicKey)
    ).slaAccountAddresses;

    expect(
      actualSlaAccountAddresses[0].toString(),
      "SLA registry address doesn't match  the expected address"
    ).to.equal(expectedSlaAccountAddresses[0].toString());

    expect(
      actualSlaAccountAddresses[1].toString(),
      "SLA registry address doesn't match  the expected address"
    ).to.equal(expectedSlaAccountAddresses[1].toString());

    expect(
      actualSlaAccountAddresses.length,
      "SLA registry lenghth doesn't match"
    ).to.equal(expectedSlaAccountAddresses.length);
  });
});
