import {
  SLA_PROTOCOL_DEPLOYER,
  SLA_REGISTRY_ADDRESS,
  RANDOM_MINT,
  DSLA_MINT,
  //   SLA_ADDRESS,
  GOVERNANCE_SEED,
  SLAS,
  SLA_AUTHORITY_SEED,
  STATUS_REGISTRY_SEED,
  POOL_SEED,
  DSLA_POOL_SEED,
  UT_MINT_SEED,
  PT_MINT_SEED,
  AGGREGATOR_ADDRESS,
} from "./constants";
import {
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { PROGRAM_ID } from "../anchor-client-gen/programId";
import {
  Transaction,
  SystemProgram,
  Connection,
  sendAndConfirmTransaction,
  PublicKey,
  Keypair,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { deploySla } from "../anchor-client-gen/instructions";

export async function deploySlaTx(connection: Connection) {
  const sla = SLAS[0];
  const slaKeypair = Keypair.generate();
  console.log("SLA ADDRESS: ", slaKeypair.publicKey.toString());
  const tx = new Transaction();

  // PDAS
  const slaAuthorityPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SLA_AUTHORITY_SEED), slaKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  )[0];
  const statusRegistryPda = PublicKey.findProgramAddressSync(
    [Buffer.from(STATUS_REGISTRY_SEED), slaKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  )[0];
  const poolPda = PublicKey.findProgramAddressSync(
    [Buffer.from(POOL_SEED), slaKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  )[0];
  const dslaPoolPda = PublicKey.findProgramAddressSync(
    [Buffer.from(DSLA_POOL_SEED), slaKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  )[0];

  const governancePda = PublicKey.findProgramAddressSync(
    [Buffer.from(GOVERNANCE_SEED)],
    PROGRAM_ID
  )[0];
  const utMintPda = PublicKey.findProgramAddressSync(
    [Buffer.from(UT_MINT_SEED), slaKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  )[0];
  const ptMintPda = PublicKey.findProgramAddressSync(
    [Buffer.from(PT_MINT_SEED), slaKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  )[0];

  /// TOKEN ACCOUNTS
  let deployerDslaTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    DSLA_MINT, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );

  tx.add(
    deploySla(
      { ...sla },
      {
        deployer: SLA_PROTOCOL_DEPLOYER.publicKey,
        slaRegistry: SLA_REGISTRY_ADDRESS.publicKey,
        sla: slaKeypair.publicKey,
        slaAuthority: slaAuthorityPda,
        statusRegistry: statusRegistryPda,
        mint: RANDOM_MINT,
        pool: poolPda,
        dslaMint: DSLA_MINT,
        dslaPool: dslaPoolPda,
        /** The token account to pay the DSLA fee from */
        deployerDslaTokenAccount: deployerDslaTokenAccount.address,
        governance: governancePda,
        utMint: utMintPda,
        ptMint: ptMintPda,
        aggregator: AGGREGATOR_ADDRESS,
        /** The program for interacting with the token. */
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      }
    )
  );

  return await sendAndConfirmTransaction(connection, tx, [
    SLA_PROTOCOL_DEPLOYER,
    slaKeypair,
  ]);
}

// export async function fetch_sla_account(connection: Connection) {
//   console.log();
//   const acc = await Sla.fetch(connection, SLA_ADDRESS);
//   if (acc === null) {
//     // the fetch method returns null when the account is uninitialized
//     console.log("account not found");
//   }
//   // convert to a JSON object
//   const obj = acc.toJSON();
//   return obj;
// }
