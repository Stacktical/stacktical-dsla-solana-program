import {
  SLA_PROTOCOL_DEPLOYER,
  SLA_REGISTRY_DEPLOYER,
  SLA_DEPLOYERS,
  STAKERS,
  MINT_AUTHORITY,
  DSLA_MINT_AUTHORITY,
  PROVIDER,
} from "./constants";
import { fund_account } from "./utils";
import * as anchor from "@project-serum/anchor";
import { createMint } from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import { Dsla } from "../target/types/dsla";
import { PublicKey } from "@solana/web3.js";
import { SwitchboardTestContext } from "@switchboard-xyz/sbv2-utils";
import { AggregatorAccount } from "@switchboard-xyz/switchboard-v2";
import { mintToChecked } from "@solana/spl-token";
import { createAssociatedTokenAccount } from "@solana/spl-token";

// Configure the client to use the local cluster.
anchor.setProvider(PROVIDER);
export const connection: anchor.web3.Connection = PROVIDER.connection;
export const program: Program<Dsla> = anchor.workspace.Dsla as Program<Dsla>;
export var mint: PublicKey;
export var dsla_mint: PublicKey;
export var aggregatorAccount: AggregatorAccount;
export var switchboard: SwitchboardTestContext;
// Will run after every test in every file
before(async () => {
  await fund_account(connection, SLA_PROTOCOL_DEPLOYER.publicKey);
  await fund_account(connection, SLA_REGISTRY_DEPLOYER.publicKey);
  await fund_account(connection, MINT_AUTHORITY.publicKey);
  await fund_account(connection, DSLA_MINT_AUTHORITY.publicKey);
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
    8 // decimals
  );
  SLA_DEPLOYERS.forEach(async (keypair) => {
    await fund_account(connection, keypair.publicKey);

    let ata = await createAssociatedTokenAccount(
      connection, // connection
      keypair, // fee payer
      dsla_mint, // mint
      keypair.publicKey // owner,
    );
    await mintToChecked(
      connection,
      keypair,
      dsla_mint,
      ata,
      DSLA_MINT_AUTHORITY,
      1_000_000_000_000,
      8
    );
  });

  // load the Switchboard env to dictate which queue to create feed for
  // switchboard = await SwitchboardTestContext.loadFromEnv(
  //   anchor.AnchorProvider.env()
  // );
  // create a static feed that will always resolve to 100
  // then call openRound and wait for the oracle to process the update
  // aggregatorAccount = await switchboard.createStaticFeed(100);
});
