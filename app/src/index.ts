import { Connection } from '@solana/web3.js';
import * as dotenv from 'dotenv';
// import { fetch_governance_account, initGovernanceTx } from './init_governance';
dotenv.config();

async function main() {
  const connection = new Connection(process.env.RPC_PROVIDER);
  // initGovernanceTx(connection);
  // console.log(await fetch_governance_account(connection));
}

main();
