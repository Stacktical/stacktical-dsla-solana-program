import {
  SLA_PROTOCOL_DEPLOYER,
  SLA_REGISTRY_DEPLOYER,
  SLA_DEPLOYERS,
  STAKERS,
  MINT_AUTHORITY,
  DSLA_MINT_AUTHORITY,
} from "./constants";
import { fund_account } from "./utils";
import * as anchor from "@project-serum/anchor";
import { createMint } from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import { Dsla } from "../target/types/dsla";
import { PublicKey } from "@solana/web3.js";

// Configure the client to use the local cluster.
let provider = anchor.AnchorProvider.local();
// Configure the client to use the local cluster.
anchor.setProvider(provider);
export const connection: anchor.web3.Connection = provider.connection;
export const program: Program<Dsla> = anchor.workspace.Dsla as Program<Dsla>;
export var mint: PublicKey;
export var dsla_mint: PublicKey;

// Will run after every test in every file
before(async function () {
  await fund_account(connection, SLA_PROTOCOL_DEPLOYER.publicKey);
  await fund_account(connection, SLA_REGISTRY_DEPLOYER.publicKey);
  await fund_account(connection, MINT_AUTHORITY.publicKey);
  await fund_account(connection, DSLA_MINT_AUTHORITY.publicKey);

  SLA_DEPLOYERS.forEach(async (keypair) => {
    await fund_account(connection, keypair.publicKey);
  });
  STAKERS.forEach(async (keypair) => {
    await fund_account(connection, keypair.publicKey);
  });

  mint = await createMint(
    connection,
    MINT_AUTHORITY, // fee payer
    MINT_AUTHORITY.publicKey, // mint authority
    MINT_AUTHORITY.publicKey, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
    8 // decimals
  );

  dsla_mint = await createMint(
    connection,
    DSLA_MINT_AUTHORITY, // fee payer
    DSLA_MINT_AUTHORITY.publicKey, // mint authority
    DSLA_MINT_AUTHORITY.publicKey, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
    18 // decimals
  );
});
