import { SLA_REGISTRY_DEPLOYER, SLA_DEPLOYERS, STAKERS } from "./constants";
import { fund_account } from "./utils";
import * as anchor from "@project-serum/anchor";

describe("Initialize Lockup accounts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;

  it("funds the accounts", async () => {
    await fund_account(connection, SLA_REGISTRY_DEPLOYER.publicKey);
    SLA_DEPLOYERS.forEach(async (keypair) => {
      await fund_account(connection, keypair.publicKey);
    });

    STAKERS.forEach(async (keypair) => {
      await fund_account(connection, keypair.publicKey);
    });
  });
});
