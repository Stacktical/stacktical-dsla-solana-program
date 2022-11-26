import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { NATIVE_MINT } from "@solana/spl-token";
import { STAKERS, SLA_KEYPAIRS, PT_MINT_SEED } from "./constants";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { fund_account } from "./utils";
import { mintToChecked } from "@solana/spl-token";
import { transferChecked } from "@solana/spl-token";

describe("Stake", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;

  it("stakes provider side", async () => {
    const token_amount = new anchor.BN(LAMPORTS_PER_SOL * 10);

    let stakerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection, // connection
      STAKERS[0], // fee payer
      NATIVE_MINT, // mint
      STAKERS[0].publicKey // owner,
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
          mint: NATIVE_MINT,
        })
        .signers([STAKERS[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }
  });
});
