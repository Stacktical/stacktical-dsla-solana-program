import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { SloRegistry } from "../target/types/slo_registry";

describe("slo-registry", async () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SloRegistry as Program<SloRegistry>;

  it("register SLO", async () => {
    let sla_address = new PublicKey(
      "AXJ1hE87vEFemyqYdxeRoWhC2z4QydB9VWYtCmqL3uT2"
    );

    let authority_address = anchor.getProvider().wallet.publicKey;
    const [sloPDA, _] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("slo"),
        authority_address.toBuffer(),
        sla_address.toBuffer(),
      ],
      program.programId
    );

    await program.rpc.registerSlo(
      sla_address,
      { greaterThan: {} },
      new BN("100000"),
      {
        accounts: {
          authority: anchor.getProvider().wallet.publicKey,
          slo: sloPDA,
          systemProgram: SystemProgram.programId,
        },
      }
    );

    // expect((await program.account.slo.fetch(sloPDA)).sloValue).to.equal(100000);

    // expect((await program.account.slo.fetch(sloPDA)).sloType).to.equal({
    //   SLOType: "GreaterThan",
    // });
  });
});
