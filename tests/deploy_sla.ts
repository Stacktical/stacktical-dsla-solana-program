import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import {
  SLA_DEPLOYERS,
  SLA_REGISTRY_KEYPAIR,
  SLA_KEYPAIRS,
  SLAS,
} from "./constants";
import { program, mint } from "./init";

describe("Deploy SLA", () => {
  let expectedSlaAccountAddresses = [];

  SLAS.forEach((sla) => {
    it(`Deploys SLA ${sla.id}`, async () => {
      // DEPLOY SLA
      try {
        await program.methods
          .deploySla(
            sla.slo,
            sla.messengerAddress,
            sla.leverage,
            sla.start,
            sla.nPeriods,
            sla.periodLength
          )
          .accounts({
            deployer: SLA_DEPLOYERS[sla.id].publicKey,
            slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
            sla: SLA_KEYPAIRS[sla.id].publicKey,
            mint: mint,
          })
          .signers([SLA_DEPLOYERS[sla.id], SLA_KEYPAIRS[sla.id]])
          .rpc();
      } catch (err) {
        console.log(err);
      }

      // VERIFY CORRECT DEPLOYMENT
      expectedSlaAccountAddresses.push(SLA_KEYPAIRS[sla.id].publicKey);
      const actualSlaAccountAddresses = (
        await program.account.slaRegistry.fetch(SLA_REGISTRY_KEYPAIR.publicKey)
      ).slaAccountAddresses;

      expect(
        actualSlaAccountAddresses[sla.id].toString(),
        "SLA registry address doesn't match  the expected address"
      ).to.equal(expectedSlaAccountAddresses[sla.id].toString());

      expect(
        actualSlaAccountAddresses.length,
        "SLA registry lenghth doesn't match"
      ).to.equal(expectedSlaAccountAddresses.length);

      expect(
        actualSlaAccountAddresses[sla.id].toString(),
        "match to wrong address"
      ).to.not.equal(
        SLA_KEYPAIRS[(sla.id + 1) % SLA_KEYPAIRS.length].publicKey.toString()
      );
    });
  });
});
