/* eslint-disable unicorn/no-process-exit */
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import {
  AggregatorAccount,
  SwitchboardProgram,
} from "@switchboard-xyz/solana.js";
import { SLA_PROTOCOL_DEPLOYER } from "./constants";

// SOL/USD Feed https://switchboard.xyz/explorer/2/GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR
// Create your own feed here https://publish.switchboard.xyz/
const switchboardFeed = new PublicKey(
  "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR"
);

export async function create_aggregator_account(connection: Connection) {
  // load the switchboard program
  const program = await SwitchboardProgram.load(
    "devnet",
    new Connection(clusterApiUrl("devnet")),
    SLA_PROTOCOL_DEPLOYER // using dummy keypair since we wont be submitting any transactions
  );

  // load the switchboard aggregator
  const aggregator = new AggregatorAccount(program, switchboardFeed);

  console.log("aggreagator account: ", aggregator.publicKey.toString());
  // get the result
  const result = await aggregator.fetchLatestValue();
  console.log(`Switchboard Result: ${result}`);
}
