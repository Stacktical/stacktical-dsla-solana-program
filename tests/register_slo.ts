import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("register SLO", async () => {
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let provider_wallet = provider.wallet.publicKey;

  const program: Program<Dsla> = anchor.workspace.Dsla;

  it("register SLO", async () => {
    const sla_address = new PublicKey(
      "AXJ1hE87vEFemyqYdxeRoWhC2z4QydB9VWYtCmqL3uT2"
    );

    let slo_value = new BN("100000");
    let slo_type = { greaterThan: {} };
    const [sloPDA, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("slo"),
        provider_wallet.toBuffer(),
        sla_address.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .registerSlo(sla_address, slo_type, slo_value)
      .accounts({
        owner: provider_wallet,
        slo: sloPDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    expect(
      (await program.account.slo.fetch(sloPDA)).sloValue.toNumber(),
      "SLO value does not match the input"
    ).to.equal(slo_value.toNumber());

    expect(
      (await program.account.slo.fetch(sloPDA)).sloType,
      "SLO type does not match the input"
    ).to.deep.equal(slo_type);
  });

  it("should fail to register the same SLO for the same SLA twice", async () => {
    const sla_address = new PublicKey(
      "AXJ1hE87vEFemyqYdxeRoWhC2z4QydB9VWYtCmqL3uT1"
    );
    let slo_value1 = new BN("999999");
    let slo_type1 = { smallerThan: {} };
    const [sloPDA, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("slo"),
        provider_wallet.toBuffer(),
        sla_address.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .registerSlo(sla_address, slo_type1, slo_value1)
      .accounts({
        owner: provider_wallet,
        slo: sloPDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let slo_value2 = new BN("100000");
    let slo_type2 = { greaterThan: {} };

    try {
      await program.methods
        .registerSlo(sla_address, slo_type2, slo_value2)
        .accounts({
          owner: provider_wallet,
          slo: sloPDA,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      expect(false, "should return error");
    } catch (e) {
      expect(e, "wrong error returned").to.have.property("programErrorStack");
    }

    expect(
      (await program.account.slo.fetch(sloPDA)).sloValue.toNumber(),
      "SLO value does not match the input"
    ).to.equal(slo_value1.toNumber());

    expect(
      (await program.account.slo.fetch(sloPDA)).sloType,
      "SLO type does not match the input"
    ).to.deep.equal(slo_type1);
  });
});
