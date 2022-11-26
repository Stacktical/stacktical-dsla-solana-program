import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Dsla } from "../target/types/dsla";

import { SystemProgram, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { SLA_KEYPAIRS, STAKERS } from "./constants";

describe("Initialize Lockup accounts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.Dsla as Program<Dsla>;

  it("initializes the UT and PT accounts", async () => {
    try {
      await program.methods
        .initLockupAccounts()
        .accounts({
          userProvider: STAKERS[0].publicKey,
          sla: SLA_KEYPAIRS[0].publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([STAKERS[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }
  });
});
