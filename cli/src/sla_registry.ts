import { initSlaRegistry } from "../anchor-client-gen/instructions";
import {
  SLA_PROTOCOL_DEPLOYER,
  SLA_REGISTRY_SPACE,
  SLA_REGISTRY_ADDRESS,
} from "./constants";
import {
  Transaction,
  SystemProgram,
  Connection,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";
import { SlaRegistry } from "../anchor-client-gen/accounts";
import { PROGRAM_ID } from "../anchor-client-gen/programId";

export async function initSlaRegistryTx(connection: Connection) {
  const slaRegistryKeypair = Keypair.generate();

  // Seed the created account with lamports for rent exemption
  const rentExemptionAmount =
    await connection.getMinimumBalanceForRentExemption(SLA_REGISTRY_SPACE);

  // call an instruction
  const tx = new Transaction();
  console.log(
    "sla registry public key: ",
    slaRegistryKeypair.publicKey.toString()
  );
  console.log(
    "sla registry secret key: ",
    slaRegistryKeypair.secretKey.toString()
  );
  tx.add(
    SystemProgram.createAccount({
      fromPubkey: SLA_PROTOCOL_DEPLOYER.publicKey,
      newAccountPubkey: slaRegistryKeypair.publicKey,
      lamports: rentExemptionAmount,
      space: SLA_REGISTRY_SPACE,
      programId: PROGRAM_ID,
    })
  );
  tx.add(
    initSlaRegistry({
      deployer: SLA_PROTOCOL_DEPLOYER.publicKey,
      slaRegistry: slaRegistryKeypair.publicKey,
      systemProgram: SystemProgram.programId,
    })
  );

  return await sendAndConfirmTransaction(connection, tx, [
    SLA_PROTOCOL_DEPLOYER,
    slaRegistryKeypair,
  ]);
}

export async function fetch_sla_registry_account(connection: Connection) {
  const acc = await SlaRegistry.fetch(
    connection,
    SLA_REGISTRY_ADDRESS.publicKey
  );
  if (acc === null) {
    console.log("account not found");
  }
  // convert to a JSON object
  const obj = acc.toJSON();
  return obj;
}
