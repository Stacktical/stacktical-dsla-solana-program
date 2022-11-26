import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import { NATIVE_MINT } from "@solana/spl-token";
import { SLA_DEPLOYERS, SLA_REGISTRY_KEYPAIR, SLA_KEYPAIRS } from "./constants";
describe("Deploy SLA", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.Dsla as Program<Dsla>;

  it("Deploys an SLA 1", async () => {
    let now = Date.now() + 1000 * 1;

    let start = new anchor.BN(now);
    let nPeriods = new anchor.BN("100");

    let length = new anchor.BN(1000 * 60 * 60);
    let periodLength = { custom: { length } };

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
          nPeriods,
          periodLength
        )
        .accounts({
          deployer: SLA_DEPLOYERS[0].publicKey,
          slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
          sla: SLA_KEYPAIRS[0].publicKey,
          mint: NATIVE_MINT,
        })
        .signers([SLA_DEPLOYERS[0], SLA_KEYPAIRS[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    const expectedSlaAccountAddresses = [SLA_KEYPAIRS[0].publicKey];
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
    ).to.not.equal(SLA_KEYPAIRS[1].publicKey.toString());
  });

  it("Deploys an SLA 2", async () => {
    let start = new anchor.BN("10000000");
    let nPeriods = new anchor.BN("5");
    let length = new anchor.BN("10000");
    let periodLength = { custom: { length } };

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
          nPeriods,
          periodLength
        )
        .accounts({
          deployer: SLA_DEPLOYERS[1].publicKey,
          slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
          sla: SLA_KEYPAIRS[1].publicKey,
          mint: NATIVE_MINT,
        })
        .signers([SLA_DEPLOYERS[1], SLA_KEYPAIRS[1]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    const expectedSlaAccountAddresses = [
      SLA_KEYPAIRS[0].publicKey,
      SLA_KEYPAIRS[1].publicKey,
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
