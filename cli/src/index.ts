import * as dotenv from "dotenv";
import { Connection } from "@solana/web3.js";
import yargs from "yargs/yargs";
import { hideBin } from "yargs/helpers";
import { fetch_governance_account, initGovernanceTx } from "./governance";
import { create_aggregator_account } from "./switchboard";
import { initSlaRegistryTx, fetch_sla_registry_account } from "./sla_registry";
import { deploySlaTx } from "./sla";
import { initLockupAccountsTx } from "./lockup_accounts";
import { stakerProviderTx } from "./stake_provider";
import { stakerUserTx } from "./stake_user";
import { validatePeriodTx } from "./validate";

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
  } else if (argv["get_feed_data"]) {
    await create_aggregator_account(connection);
    console.log("success");
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
  } else if (argv["init_lockup_accounts"]) {
    let tx = await initLockupAccountsTx(connection);
    console.log("initialized lockup accounts successfully: ", tx);
  } else if (argv["stake_provider"]) {
    let tx = await stakerProviderTx(connection);
    console.log("staked successfully: ", tx);
  } else if (argv["stake_user"]) {
    let tx = await stakerUserTx(connection);
    console.log("staked successfully: ", tx);
  } else if (argv["validate_period"]) {
    let tx = await validatePeriodTx(connection);
    console.log("staked successfully: ", tx);
  } else {
    console.log(`received no command`);
  }
}

main();
