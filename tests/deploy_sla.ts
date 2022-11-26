import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import { Keypair } from "@solana/web3.js";
import { NATIVE_MINT } from "@solana/spl-token";
import { DEPLOYER, SLA_REGISTRY_KEYPAIR } from "./constants";
describe("Deploy SLA", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.Dsla as Program<Dsla>;

  const slaKeypairs = [
    Keypair.generate(),
    Keypair.generate(),
    Keypair.generate(),
  ];

  it("Deploys an SLA 1", async () => {
    let start = new anchor.BN("7000000");
    let n_periods = new anchor.BN("100");
    let period_length = { custom: { lenght: 128 } };

    let sloType = { greaterThan: {} };
    let sloValue = {
      mantissa: new anchor.BN("100"),
      scale: new anchor.BN("0"),
    };
    const slo = {
      sloValue,
      sloType,
    };
    const messengerAddress = anchor.web3.Keypair.generate().publicKey;

    const leverage = new anchor.BN("1");

    try {
      await program.methods
        .deploySla(
          slo,
          messengerAddress,
          leverage,
          start,
          n_periods,
          period_length
        )
        .accounts({
          deployer: DEPLOYER.publicKey,
          slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
          sla: slaKeypairs[0].publicKey,
          mint: NATIVE_MINT,
        })
        .signers([DEPLOYER, slaKeypairs[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    const expectedSlaAccountAddresses = [slaKeypairs[0].publicKey];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(SLA_REGISTRY_KEYPAIR.publicKey)
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
    let start = new anchor.BN("10000000");
    let n_periods = new anchor.BN("5");
    let period_length = { custom: { lenght: 10000 } };

    let sloType = { smallerThan: {} };
    let sloValue = {
      mantissa: new anchor.BN("100"),
      scale: new anchor.BN("0"),
    };
    const slo = {
      sloValue,
      sloType,
    };
    const messengerAddress = anchor.web3.Keypair.generate().publicKey;

    const leverage = new anchor.BN("5");

    try {
      await program.methods
        .deploySla(
          slo,
          messengerAddress,
          leverage,
          start,
          n_periods,
          period_length
        )
        .accounts({
          deployer: DEPLOYER.publicKey,
          slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
          sla: slaKeypairs[1].publicKey,
          mint: NATIVE_MINT,
        })
        .signers([DEPLOYER, slaKeypairs[1]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    const expectedSlaAccountAddresses = [
      slaKeypairs[0].publicKey,
      slaKeypairs[1].publicKey,
    ];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(SLA_REGISTRY_KEYPAIR.publicKey)
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
