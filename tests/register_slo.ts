import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("register SLO", async () => {
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let owner = provider.wallet.publicKey;

  const program: Program<Dsla> = anchor.workspace.Dsla;

  const [slo, _bump] = await PublicKey.findProgramAddress(
    [anchor.utils.bytes.utf8.encode("slo"), owner.toBuffer()],
    program.programId
  );

  let systemProgram = SystemProgram.programId;

  let slo_values = [new BN("100000"), new BN("100000")];
  let slo_types = [{ greaterThan: {} }, { equalTo: {} }];

  it("register SLO", async () => {
    await program.methods
      .registerSlo(slo_types[0], slo_values[0])
      .accounts({
        owner,
        slo,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    expect(
      (await program.account.slo.fetch(slo)).sloValue.toNumber(),
      "SLO value does not match the input"
    ).to.equal(slo_values[0].toNumber());

    expect(
      (await program.account.slo.fetch(slo)).sloType,
      "SLO type does not match the input"
    ).to.deep.equal(slo_types[0]);
  });

  it("should fail to register the same SLO for the same SLA twice", async () => {
    try {
      await program.methods
        .registerSlo(slo_types[1], slo_values[1])
        .accounts({
          owner,
          slo,
          systemProgram,
        })
        .rpc();
      expect(false, "should return error");
    } catch (e) {
      expect(e, "wrong error returned").to.have.property("programErrorStack");
    }

    expect(
      (await program.account.slo.fetch(slo)).sloValue.toNumber(),
      "SLO value does not match the input"
    ).to.equal(slo_values[0].toNumber());

    expect(
      (await program.account.slo.fetch(slo)).sloType,
      "SLO type does not match the input"
    ).to.deep.equal(slo_types[0]);
  });
});
