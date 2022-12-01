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
import { BN } from "bn.js";

describe("Stake Provider", () => {
  it("stakes provider side", async () => {
    const tokenAmount = new anchor.BN(LAMPORTS_PER_SOL * 10);

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
    let stakerPtAccountAmount = new anchor.BN(stakerPtAccount.amount);
    expect(
      stakerPtAccountAmount.eq(new anchor.BN(0)),
      "provider token account is not empty"
    ).to.be.true;

    let providerPoolSize = (
      await program.account.sla.fetch(SLA_KEYPAIRS[0].publicKey)
    ).providerPoolSize;
    expect(providerPoolSize.eq(new anchor.BN(0)), "provider pool is not 0").to
      .be.true;
    expect(providerPoolSize.gt(new anchor.BN(0)), "provider pool is not 0").to
      .be.false;

    try {
      await program.methods
        .stakeProvider(tokenAmount)
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

    stakerPtAccountAmount = new anchor.BN(
      (
        await getOrCreateAssociatedTokenAccount(
          connection,
          STAKERS[0], // fee payer
          ptMintPda, // mint
          STAKERS[0].publicKey // owner,
        )
      ).amount
    );
    expect(
      stakerPtAccountAmount.eq(new anchor.BN(tokenAmount)),
      "provider token account amount does not equal staked token Amount"
    ).to.be.true;

    providerPoolSize = (
      await program.account.sla.fetch(SLA_KEYPAIRS[0].publicKey)
    ).providerPoolSize;

    expect(
      providerPoolSize.eq(tokenAmount),
      "provider pool size is not equal to the staked token amount size"
    ).to.be.true;
    expect(
      providerPoolSize.lt(tokenAmount),
      "provider pool size is not equal to the staked token amount size"
    ).to.be.false;

    // lockupAccount = await program.account.lockup.fetch(SLA_KEYPAIRS[0].publicKey);
  });
});
