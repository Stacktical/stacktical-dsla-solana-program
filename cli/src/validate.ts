import { validatePeriod } from "../anchor-client-gen/instructions";
import { PROGRAM_ID } from "../anchor-client-gen/programId";
import {
  SLA_PROTOCOL_DEPLOYER,
  SLA_ADDRESS,
  RANDOM_MINT,
  SLA_AUTHORITY_SEED,
  AGGREGATOR_ADDRESS,
  DSLA_MINT,
  STATUS_REGISTRY_SEED,
  GOVERNANCE_SEED,
  DSLA_POOL_SEED,
  SLAS,
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

export async function validatePeriodTx(connection: Connection) {
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
  const statusRegistryPda = PublicKey.findProgramAddressSync(
    [Buffer.from(STATUS_REGISTRY_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];
  const governancePda = PublicKey.findProgramAddressSync(
    [Buffer.from(GOVERNANCE_SEED)],
    PROGRAM_ID
  )[0];
  const dslaPoolPda = PublicKey.findProgramAddressSync(
    [Buffer.from(DSLA_POOL_SEED), SLA_ADDRESS.toBuffer()],
    PROGRAM_ID
  )[0];

  /// TOKEN ACCOUNTS
  let validatorDslaTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    DSLA_MINT, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );
  let protocolDslaTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, // connection
    SLA_PROTOCOL_DEPLOYER, // fee payer
    DSLA_MINT, // mint
    SLA_PROTOCOL_DEPLOYER.publicKey // owner,
  );

  console.log(SLAS[0].start.toString());
  tx.add(
    validatePeriod(
      {
        period: new BN(0),
      },
      {
        validator: SLA_PROTOCOL_DEPLOYER.publicKey,
        slaAuthority: slaAuthorityPda,
        statusRegistry: statusRegistryPda,
        sla: SLA_ADDRESS,
        aggregator: AGGREGATOR_ADDRESS,
        governance: governancePda,
        dslaMint: DSLA_MINT,
        dslaPool: dslaPoolPda,
        validatorDslaTokenAccount: validatorDslaTokenAccount.address,
        program: PROGRAM_ID,
        programData: programDataPda,
        protocolDslaTokenAccount: protocolDslaTokenAccount.address,
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
