// import { PublicKey } from "@solana/web3.js";
import { SLA_PROTOCOL_DEPLOYER, GOVERNANCE_PARAMETERS } from "./constants";
import { program } from "./init";
import { AnchorError, BN } from "@project-serum/anchor";
import { expect } from "chai";

describe("Initialize the Governance PDA", () => {
  // let programDataAddress;
  // before(async () => {
  //   [programDataAddress] = await PublicKey.findProgramAddress(
  //     [program.programId.toBuffer()],
  //     program.programId
  //   );
  // });
  it("should fail to initialize governance PDA", async () => {
    let gov_params = { ...GOVERNANCE_PARAMETERS };
    gov_params.dslaProtocolReward = gov_params.dslaProtocolReward.add(
      new BN(1)
    );
    try {
      await program.methods
        .initGovernance(gov_params)
        .accounts({
          programUpgradeAuthority: SLA_PROTOCOL_DEPLOYER.publicKey,
          // programData: programDataAddress,
          // program: program.programId,
        })
        .signers([SLA_PROTOCOL_DEPLOYER])
        .rpc();
      chai.assert(false, "should've failed but didn't ");
    } catch (_err) {
      expect(_err).to.be.instanceOf(AnchorError);
      const err: AnchorError = _err;
      expect(err.error.errorCode.code).to.equal("NonValidGovernanceParameters");
      expect(err.error.errorCode.number).to.equal(6009);
      expect(err.program.equals(program.programId)).is.true;
    }
  });

  it("initialize governance PDA", async () => {
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
