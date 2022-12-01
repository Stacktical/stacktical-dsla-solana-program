import { PublicKey } from "@solana/web3.js";
import { SLA_PROTOCOL_DEPLOYER, GOVERNANCE_PARAMETERS } from "./constants";
import { program } from "./init";

describe("Initialize the Governance PDA", () => {
  it("initialize governance PDA", async () => {
    // const [programDataAddress] = await PublicKey.findProgramAddress(
    //   [program.programId.toBuffer()],
    //   program.programId
    // );

    await program.methods
      .initGovernance(GOVERNANCE_PARAMETERS)
      .accounts({
        programUpgradeAuthority: SLA_PROTOCOL_DEPLOYER.publicKey,
        // programData: programDataAddress,
        // program: program.programId,
      })
      .signers([SLA_PROTOCOL_DEPLOYER])
      .rpc();
  });
});
