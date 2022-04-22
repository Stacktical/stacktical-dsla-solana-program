import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import {
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
chai.use(chaiAsPromised);

describe("Create SLA", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.Dsla as Program<Dsla>;
  const programProvider = program.provider as anchor.AnchorProvider;
  const slaRegistryKeypair = anchor.web3.Keypair.generate();
  const messangerAddress = anchor.web3.Keypair.generate().publicKey;

  const deployer = programProvider.wallet;

  const createAccountParams = {
    fromPubkey: deployer.publicKey,
    newAccountPubkey: slaRegistryKeypair.publicKey,
    lamports: 10000000,
    space: 10000000,
    programId: program.programId,
  };

  const createAccountTransaction = new Transaction().add(
    SystemProgram.createAccount(createAccountParams)
  );

  await sendAndConfirmTransaction(
    provider.connection,
    createAccountTransaction,
    [slaRegistryKeypair]
  );
});
