// import { PublicKey } from "@solana/web3.js";
import {
  SLA_PROTOCOL_DEPLOYER,
  GOVERNANCE_PARAMETERS,
  STAKERS,
} from "./constants";
import { program } from "./init";
import { AnchorError, BN, web3 } from "@project-serum/anchor";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("Initialize the Governance PDA", () => {
  let programDataAddress: PublicKey;
  before(async () => {
    programDataAddress = (
      await PublicKey.findProgramAddress(
        [program.programId.toBuffer()],
        new web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
      )
    )[0];
  });

  it("should fail to initialize governance PDA because it's not the program upgrade authority", async () => {
    try {
      await program.methods
        .initGovernance(
          GOVERNANCE_PARAMETERS.dslaDepositByPeriod,
          GOVERNANCE_PARAMETERS.dslaProtocolReward,
          GOVERNANCE_PARAMETERS.dslaValidatorReward,
          GOVERNANCE_PARAMETERS.dslaBurnedByVerification,
          GOVERNANCE_PARAMETERS.slaDeployerRewardsRate,
          GOVERNANCE_PARAMETERS.protocolRewardsRate,
          GOVERNANCE_PARAMETERS.maxLeverage
        )
        .accounts({
          programUpgradeAuthority: STAKERS[0].publicKey,
          programData: programDataAddress,
          program: program.programId,
        })
        .signers([STAKERS[0]])
        .rpc();
      chai.assert(false, "should've failed but didn't ");
    } catch (_err) {
      expect(_err).to.be.instanceOf(AnchorError);
      const err: AnchorError = _err;
      expect(err.error.errorCode.code).to.equal("ConstraintRaw");
      expect(err.error.errorCode.number).to.equal(2003);
      expect(err.program.equals(program.programId)).is.true;
    }
  });

  it("should fail to initialize governance PDA because of bad parameters", async () => {
    let gov_params = { ...GOVERNANCE_PARAMETERS };
    gov_params.dslaProtocolReward = gov_params.dslaProtocolReward.add(
      new BN(1)
    );

    try {
      await program.methods
        .initGovernance(
          gov_params.dslaDepositByPeriod,
          gov_params.dslaProtocolReward,
          gov_params.dslaValidatorReward,
          gov_params.dslaBurnedByVerification,
          gov_params.slaDeployerRewardsRate,
          gov_params.protocolRewardsRate,
          gov_params.maxLeverage
        )
        .accounts({
          programUpgradeAuthority: SLA_PROTOCOL_DEPLOYER.publicKey,
          programData: programDataAddress,
          program: program.programId,
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
      .initGovernance(
        GOVERNANCE_PARAMETERS.dslaDepositByPeriod,
        GOVERNANCE_PARAMETERS.dslaProtocolReward,
        GOVERNANCE_PARAMETERS.dslaValidatorReward,
        GOVERNANCE_PARAMETERS.dslaBurnedByVerification,
        GOVERNANCE_PARAMETERS.slaDeployerRewardsRate,
        GOVERNANCE_PARAMETERS.protocolRewardsRate,
        GOVERNANCE_PARAMETERS.maxLeverage
      )
      .accounts({
        programUpgradeAuthority: SLA_PROTOCOL_DEPLOYER.publicKey,
        programData: programDataAddress,
        program: program.programId,
      })
      .signers([SLA_PROTOCOL_DEPLOYER])
      .rpc();
  });
});
