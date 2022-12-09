import * as dotenv from "dotenv";
import { Connection } from "@solana/web3.js";
import yargs from "yargs/yargs";
import { hideBin } from "yargs/helpers";
import { fetch_governance_account, initGovernanceTx } from "./governance";
import { create_aggregator_account, read_feed } from "./switchboard";
import { initSlaRegistryTx, fetch_sla_registry_account } from "./sla_registry";
import { deploySlaTx } from "./sla";
dotenv.config();

async function main() {
  const connection = new Connection(process.env.RPC_PROVIDER);

  const argv = yargs(hideBin(process.argv)).argv;
  if (argv["init_governance"]) {
    let tx = await initGovernanceTx(connection);
    console.log(
      "initialized governance successfully with transaction id: ",
      tx
    );
  } else if (argv["print_governance"]) {
    console.log(await fetch_governance_account(connection));
  } else if (argv["create_aggregator_account"]) {
    let aggregatorAccount = await create_aggregator_account(connection);
    console.log(aggregatorAccount.publicKey.toString());
    console.log(await read_feed(aggregatorAccount));
  } else if (argv["init_sla_registry"]) {
    let tx = await initSlaRegistryTx(connection);
    console.log(
      "initialized SLA registry successfully with transaction id: ",
      tx
    );
  } else if (argv["print_sla_registry"]) {
    console.log(await fetch_sla_registry_account(connection));
  } else if (argv["deploy_sla"]) {
    let tx = await deploySlaTx(connection);
    console.log("deployed SLA successfully with transaction id: ", tx);
  } else {
    console.log(`received no command`);
  }
}

main();
