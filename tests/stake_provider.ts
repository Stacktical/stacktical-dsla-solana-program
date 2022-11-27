import * as anchor from "@project-serum/anchor";
import { expect } from "chai";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  STAKERS,
  SLA_KEYPAIRS,
  PT_MINT_SEED,
  MINT_AUTHORITY,
} from "./constants";
import {
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
} from "@solana/spl-token";
import { fund_account } from "./utils";
import { mint, program, connection } from "./init";

describe("Stake", () => {
  it("stakes provider side", async () => {
    const token_amount = new anchor.BN(LAMPORTS_PER_SOL * 10);

    let stakerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection, // connection
      STAKERS[0], // fee payer
      mint, // mint
      STAKERS[0].publicKey // owner,
    );

    await mintToChecked(
      connection,
      STAKERS[0],
      stakerTokenAccount.mint,
      stakerTokenAccount.address,
      MINT_AUTHORITY,
      LAMPORTS_PER_SOL * 100,
      8
    );

    const [ptMintPda] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(PT_MINT_SEED),
        SLA_KEYPAIRS[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    let stakerPtAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      STAKERS[0], // fee payer
      ptMintPda, // mint
      STAKERS[0].publicKey // owner,
    );

    try {
      await program.methods
        .stakeProvider(token_amount)
        .accounts({
          staker: STAKERS[0].publicKey,
          sla: SLA_KEYPAIRS[0].publicKey,
          stakerTokenAccount: stakerTokenAccount.address,
          stakerPtAccount: stakerPtAccount.address,
          mint: mint,
        })
        .signers([STAKERS[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }
  });
});
