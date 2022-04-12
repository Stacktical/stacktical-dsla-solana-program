import * as anchor from "@project-serum/anchor";
import { Program, BN, ProgramErrorStack } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { SloRegistry } from "../target/types/slo_registry";
import { PublicKey } from "@solana/web3.js";

describe("slo-registry", async () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SloRegistry as Program<SloRegistry>;

  it("register SLO", async () => {
    const sla_address = new PublicKey(
      "AXJ1hE87vEFemyqYdxeRoWhC2z4QydB9VWYtCmqL3uT2"
    );

    let authority_address = anchor.getProvider().wallet.publicKey;
    let slo_value = new BN("100000");
    let slo_type = { greaterThan: {} };
    const [sloPDA, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("slo"),
        authority_address.toBuffer(),
        sla_address.toBuffer(),
      ],
      program.programId
    );

    await program.rpc.registerSlo(sla_address, slo_type, slo_value, {
      accounts: {
        user: authority_address,
        slo: sloPDA,
        systemProgram: SystemProgram.programId,
      },
    });

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
    let authority_address = anchor.getProvider().wallet.publicKey;
    let slo_value1 = new BN("999999");
    let slo_type1 = { smallerThan: {} };
    const [sloPDA, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("slo"),
        authority_address.toBuffer(),
        sla_address.toBuffer(),
      ],
      program.programId
    );

    await program.rpc.registerSlo(sla_address, slo_type1, slo_value1, {
      accounts: {
        user: authority_address,
        slo: sloPDA,
        systemProgram: SystemProgram.programId,
      },
    });

    let slo_value2 = new BN("100000");
    let slo_type2 = { greaterThan: {} };

    try {
      await program.rpc.registerSlo(sla_address, slo_type2, slo_value2, {
        accounts: {
          user: authority_address,
          slo: sloPDA,
          systemProgram: SystemProgram.programId,
        },
      });
      expect(false, "should return error");
    } catch (e) {
      expect(e, "wrong error returned").to.have.property("programErrorStack");
    }
  });
});
