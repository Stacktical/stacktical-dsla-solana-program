import { initLockupAccounts } from "../anchor-client-gen/instructions";
import { PROGRAM_ID } from "../anchor-client-gen/programId";
import {
  SLA_PROTOCOL_DEPLOYER,
  LOCKUP_PROVIDER_SEED,
  SLA_ADDRESS,
  LOCKUP_USER_SEED,
} from "./constants";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  Connection,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

export async function initLockupAccountsTx(connection: Connection) {
  // call an instruction
  const tx = new Transaction();

  const ptLockupPda = PublicKey.findProgramAddressSync(
    [
      SLA_PROTOCOL_DEPLOYER.publicKey.toBuffer(),
      Buffer.from(LOCKUP_PROVIDER_SEED),
      SLA_ADDRESS.toBuffer(),
    ],
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

  tx.add(
    initLockupAccounts({
      userProvider: SLA_PROTOCOL_DEPLOYER.publicKey,
      sla: SLA_ADDRESS,
      ptLockup: ptLockupPda,
      utLockup: utLockupPda,
      systemProgram: SystemProgram.programId,
    })
  );

  return await sendAndConfirmTransaction(connection, tx, [
    SLA_PROTOCOL_DEPLOYER,
  ]);
}
