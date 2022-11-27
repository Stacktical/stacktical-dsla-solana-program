import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Dsla } from "../target/types/dsla";

import { SystemProgram, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { SLA_KEYPAIRS, STAKERS, SLAS } from "./constants";
import { program, connection } from "./init";

describe("Initialize Lockup accounts", () => {
  SLAS.forEach((sla) => {
    it(`initializes the UT and PT accounts for SLA ${sla.id}`, async () => {
      STAKERS.forEach(async (staker) => {
        try {
          await program.methods
            .initLockupAccounts()
            .accounts({
              userProvider: staker.publicKey,
              sla: SLA_KEYPAIRS[sla.id].publicKey,
            })
            .signers([staker])
            .rpc();
        } catch (err) {
          console.log(err);
        }
      });
    });
  });
});
