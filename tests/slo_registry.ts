import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { SloRegistry } from "../target/types/slo_registry";
import { PublicKey } from "@solana/web3.js";

describe("slo-registry", async () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SloRegistry as Program<SloRegistry>;

  const sla_address = new PublicKey(
    "AXJ1hE87vEFemyqYdxeRoWhC2z4QydB9VWYtCmqL3uT2"
  );

  it("register SLO", async () => {
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
      (await program.account.slo.fetch(sloPDA)).sloValue.toNumber()
    ).to.equal(slo_value.toNumber());

    expect((await program.account.slo.fetch(sloPDA)).sloType).to.deep.equal({
      greaterThan: {},
    });
  });
});
