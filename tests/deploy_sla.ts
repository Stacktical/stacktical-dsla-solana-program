import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { expect } from "chai";
import { Dsla, IDL } from "../target/types/dsla";
import {
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import { encodeIdlAccount } from "@project-serum/anchor/dist/cjs/idl";
chai.use(chaiAsPromised);

describe("Deploy SLA", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;
  const slaRegistryKeypair = anchor.web3.Keypair.generate();

  const deployer = Keypair.generate();
  const sla = Keypair.generate();

  const space = 10_000_000;

  before(async function () {
    const rentExemptionAmount =
      await connection.getMinimumBalanceForRentExemption(space);

    const createAccountParams = {
      fromPubkey: deployer.publicKey,
      newAccountPubkey: slaRegistryKeypair.publicKey,
      lamports: rentExemptionAmount,
      space,
      programId: program.programId,
    };

    let airdropSignature = await connection.requestAirdrop(
      deployer.publicKey,
      LAMPORTS_PER_SOL * 1000
    );
    await connection.confirmTransaction(airdropSignature);

    const createAccountTransaction = new Transaction().add(
      SystemProgram.createAccount(createAccountParams)
    );

    await sendAndConfirmTransaction(connection, createAccountTransaction, [
      deployer,
      slaRegistryKeypair,
    ]);
  });

  it("Deploys an SLA", async () => {
    const ipfsHash = "t";
    let sloType = { greaterThan: {} };
    const slo = { sloValue: new anchor.BN("100"), sloType };
    const messengerAddress = anchor.web3.Keypair.generate().publicKey;
    const periods = [
      {
        start: new anchor.BN("1000000"),
        end: new anchor.BN("1900000"),
        status: { notVerified: {} },
      },
    ];
    const leverage = new anchor.BN("1");

    const [periodRegistryPda, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("period-registry"),
        sla.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .deploySla(ipfsHash, messengerAddress, periods, leverage)
      .accounts({
        deployer: deployer.publicKey,
        slaRegistry: slaRegistryKeypair.publicKey,
        sla: sla.publicKey,
        periodRegistry: periodRegistryPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([deployer, sla])
      .rpc();
  });
});
