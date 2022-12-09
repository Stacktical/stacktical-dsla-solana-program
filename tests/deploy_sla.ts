import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";
import {
  SLA_DEPLOYERS,
  SLA_REGISTRY_KEYPAIR,
  SLA_KEYPAIRS,
  SLAS,
} from "./constants";
import { connection, program, mint, dsla_mint } from "./init";

describe("Deploy SLA", () => {
  let expectedSlaAccountAddresses = [];

  SLAS.forEach((sla) => {
    it(`Deploys SLA ${sla.id}`, async () => {
      let deployerDslaTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        SLA_DEPLOYERS[sla.id], // fee payer
        dsla_mint, // mint
        SLA_DEPLOYERS[sla.id].publicKey // owner,
      );

      // DEPLOY SLA
      try {
        await program.methods
          .deploySla(
            sla.slo,
            sla.aggregatorAddress,
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
            dslaMint: dsla_mint,
            deployerDslaTokenAccount: deployerDslaTokenAccount.address,
            aggregator: Keypair.generate().publicKey,
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
