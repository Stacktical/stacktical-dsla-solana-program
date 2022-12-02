import * as anchor from "@project-serum/anchor";
import { BN } from "@project-serum/anchor";
import { expect } from "chai";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  STAKERS,
  SLA_KEYPAIRS,
  UT_MINT_SEED,
  MINT_AUTHORITY,
} from "./constants";
import {
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
} from "@solana/spl-token";
import { mint, program, connection } from "./init";

describe("Stake User", () => {
  it("checks that it stakes user side", async () => {
    const tokenAmount = new BN(LAMPORTS_PER_SOL * 1);

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

    const [utMintPda] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(UT_MINT_SEED),
        SLA_KEYPAIRS[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    let stakerUtAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      STAKERS[0], // fee payer
      utMintPda, // mint
      STAKERS[0].publicKey // owner,
    );

    let stakerUtAccountAmount = new BN(stakerUtAccount.amount);
    expect(
      stakerUtAccountAmount.eq(new BN(0)),
      "user token account is not empty"
    ).to.be.true;

    let slaAccount = await program.account.sla.fetch(SLA_KEYPAIRS[0].publicKey);
    expect(slaAccount.userPoolSize.eq(new BN(0)), "user pool is not 0").to.be
      .true;
    expect(slaAccount.userPoolSize.gt(new BN(0)), "user pool is not 0").to.be
      .false;

    try {
      await program.methods
        .stakeUser(tokenAmount)
        .accounts({
          staker: STAKERS[0].publicKey,
          sla: SLA_KEYPAIRS[0].publicKey,
          stakerTokenAccount: stakerTokenAccount.address,
          stakerUtAccount: stakerUtAccount.address,
          mint: mint,
        })
        .signers([STAKERS[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    stakerUtAccountAmount = new BN(
      (
        await getOrCreateAssociatedTokenAccount(
          connection,
          STAKERS[0], // fee payer
          utMintPda, // mint
          STAKERS[0].publicKey // owner,
        )
      ).amount
    );
    expect(
      stakerUtAccountAmount.eq(new BN(tokenAmount)),
      "user token account amount does not equal staked token Amount"
    ).to.be.true;

    slaAccount = await program.account.sla.fetch(SLA_KEYPAIRS[0].publicKey);
    expect(
      slaAccount.userPoolSize.eq(tokenAmount),
      "user pool size is not equal to the staked token amount size"
    ).to.be.true;
    expect(
      slaAccount.userPoolSize.lt(tokenAmount),
      "user pool size is not equal to the staked token amount size"
    ).to.be.false;
  });
});
