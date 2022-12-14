import { stakeUser } from "../anchor-client-gen/instructions";
import { PROGRAM_ID } from "../anchor-client-gen/programId";
import {
  SLA_PROTOCOL_DEPLOYER,
  LOCKUP_USER_SEED,
  SLA_ADDRESS,
  RANDOM_MINT,
  POOL_SEED,
  UT_MINT_SEED,
  SLA_AUTHORITY_SEED,
} from "./constants";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  Connection,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import BN from "bn.js";
import {
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

export async function stakerUserTx(connection: Connection) {
  // call an instruction
  const tx = new Transaction();

  const utLockupPda = PublicKey.findProgramAddressSync(
    [
      SLA_PROTOCOL_DEPLOYER.publicKey.toBuffer(),
      Buffer.from(LOCKUP_USER_SEED),
      SLA_ADDRESS.toBuffer(),
    ],
    PROGRAM_ID
  )[0];

  const poolPda = PublicKey.findProgramAddressSync(
    [Buffer.from(POOL_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];

  const utMintPda = PublicKey.findProgramAddressSync(
    [Buffer.from(UT_MINT_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];

  const slaAuthorityPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SLA_AUTHORITY_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];

  /// TOKEN ACCOUNTS
  let stakerTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    RANDOM_MINT, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );

  let stakerUtAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    utMintPda, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );

  tx.add(
    stakeUser(
      {
        tokenAmount: new BN(1_000_000_000),
      },
      {
        staker: SLA_PROTOCOL_DEPLOYER.publicKey,
        sla: SLA_ADDRESS,
        slaAuthority: slaAuthorityPda,
        mint: RANDOM_MINT,
        pool: poolPda,
        utLockup: utLockupPda,
        utMint: utMintPda,
        stakerTokenAccount: stakerTokenAccount.address,
        stakerUtAccount: stakerUtAccount.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      }
    )
  );

  return await sendAndConfirmTransaction(connection, tx, [
    SLA_PROTOCOL_DEPLOYER,
  ]);
}
