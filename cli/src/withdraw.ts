import {
  withdrawProvider,
  withdrawUser,
} from "../anchor-client-gen/instructions";
import { PROGRAM_ID } from "../anchor-client-gen/programId";
import {
  SLA_PROTOCOL_DEPLOYER,
  SLA_ADDRESS,
  RANDOM_MINT,
  SLA_AUTHORITY_SEED,
  GOVERNANCE_SEED,
  LOCKUP_PROVIDER_SEED,
  PT_MINT_SEED,
  POOL_SEED,
  UT_MINT_SEED,
  LOCKUP_USER_SEED,
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

export async function withdrawProviderTx(connection: Connection) {
  // call an instruction
  const tx = new Transaction();

  const programDataPda = PublicKey.findProgramAddressSync(
    [PROGRAM_ID.toBuffer()],
    new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
  )[0];

  const slaAuthorityPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SLA_AUTHORITY_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];

  const governancePda = PublicKey.findProgramAddressSync(
    [Buffer.from(GOVERNANCE_SEED)],
    PROGRAM_ID
  )[0];

  const ptLockupPda = PublicKey.findProgramAddressSync(
    [
      SLA_PROTOCOL_DEPLOYER.publicKey.toBuffer(),
      Buffer.from(LOCKUP_PROVIDER_SEED),
      SLA_ADDRESS.toBuffer(),
    ],
    PROGRAM_ID
  )[0];

  const poolPda = PublicKey.findProgramAddressSync(
    [Buffer.from(POOL_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];

  const ptMintPda = PublicKey.findProgramAddressSync(
    [Buffer.from(PT_MINT_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];

  /// TOKEN ACCOUNTS
  let withdrawerTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    RANDOM_MINT, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );

  let withdrawerPtAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    ptMintPda, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );
  tx.add(
    withdrawProvider(
      {
        tokenAmount: new BN(1_000_000),
      },
      {
        withdrawer: SLA_PROTOCOL_DEPLOYER.publicKey,
        sla: SLA_ADDRESS,
        slaAuthority: slaAuthorityPda,
        withdrawerTokenAccount: withdrawerTokenAccount.address,
        withdrawerPtAccount: withdrawerPtAccount.address,
        ptLockup: ptLockupPda,
        mint: RANDOM_MINT,
        pool: poolPda,
        ptMint: ptMintPda,
        governance: governancePda,
        tokenProgram: TOKEN_PROGRAM_ID,
        program: PROGRAM_ID,
        programData: programDataPda,
        protocolTokenAccount: withdrawerTokenAccount.address,
        deployerTokenAccount: withdrawerTokenAccount.address,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      }
    )
  );

  return await sendAndConfirmTransaction(connection, tx, [
    SLA_PROTOCOL_DEPLOYER,
  ]);
}

export async function withdrawUserTx(connection: Connection) {
  // call an instruction
  const tx = new Transaction();

  const programDataPda = PublicKey.findProgramAddressSync(
    [PROGRAM_ID.toBuffer()],
    new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
  )[0];

  const slaAuthorityPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SLA_AUTHORITY_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];

  const governancePda = PublicKey.findProgramAddressSync(
    [Buffer.from(GOVERNANCE_SEED)],
    PROGRAM_ID
  )[0];

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

  /// TOKEN ACCOUNTS
  let withdrawerTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    RANDOM_MINT, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );

  let withdrawerUtAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    utMintPda, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );
  tx.add(
    withdrawUser(
      {
        tokenAmount: new BN(1_000_000),
      },
      {
        withdrawer: SLA_PROTOCOL_DEPLOYER.publicKey,
        sla: SLA_ADDRESS,
        slaAuthority: slaAuthorityPda,
        withdrawerTokenAccount: withdrawerTokenAccount.address,
        withdrawerUtAccount: withdrawerUtAccount.address,
        utLockup: utLockupPda,
        mint: RANDOM_MINT,
        pool: poolPda,
        utMint: utMintPda,
        governance: governancePda,
        tokenProgram: TOKEN_PROGRAM_ID,
        program: PROGRAM_ID,
        programData: programDataPda,
        protocolTokenAccount: withdrawerTokenAccount.address,
        deployerTokenAccount: withdrawerTokenAccount.address,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      }
    )
  );

  return await sendAndConfirmTransaction(connection, tx, [
    SLA_PROTOCOL_DEPLOYER,
  ]);
}
