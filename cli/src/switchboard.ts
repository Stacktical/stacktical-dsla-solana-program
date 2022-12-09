import { Connection, PublicKey } from "@solana/web3.js";
import {
  AggregatorAccount,
  OracleQueueAccount,
  loadSwitchboardProgram,
} from "@switchboard-xyz/switchboard-v2";
import { SLA_PROTOCOL_DEPLOYER } from "./constants";
import { Big } from "big.js";

export async function create_aggregator_account(connection: Connection) {
  const program = await loadSwitchboardProgram(
    "devnet",
    connection,
    SLA_PROTOCOL_DEPLOYER
  );

  const queueAccount = new OracleQueueAccount({
    program: program,
    // devnet permissionless queue
    publicKey: new PublicKey("F8ce7MsckeZAbAGmxjJNetxYXQa9mKr9nnrC3qKubyYy"),
  });

  return await AggregatorAccount.create(program, {
    name: Buffer.from("ETH_USD"),
    batchSize: 6,
    minRequiredJobResults: 1,
    minRequiredOracleResults: 1,
    minUpdateDelaySeconds: 30,
    queueAccount,
  });
}

export async function read_feed(aggregatorAccount: AggregatorAccount) {
  const result: Big = await aggregatorAccount.getLatestValue();

  return result.toNumber();
}
