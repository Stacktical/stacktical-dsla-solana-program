import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { DslaStackticalContractsSolana } from "../target/types/dsla_stacktical_contracts_solana";
import { Keypair, PublicKey } from "@solana/web3.js";

describe("dsla-stacktical-contracts-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace
    .DslaStackticalContractsSolana as Program<DslaStackticalContractsSolana>;

  it("Is initialized!", async () => {
    const slaKeypair = anchor.web3.Keypair.generate();

    const owner = program.provider.wallet;
    // Add your test here.
    let sloValue = new BN(10);
    let sloOperand = { greater: {} };
    let timestampStart = new BN(10000);
    let duration = new BN(1000);
    const tx = await program.rpc.initializeSla(
      sloValue,
      sloOperand,
      timestampStart,
      duration,
      {
        accounts: {
          sla: slaKeypair.publicKey,
          owner: owner.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [slaKeypair],
      }
    );
  });

  it("stake!", async () => {
    const slaKeypair = anchor.web3.Keypair.generate();

    const owner = anchor.web3.Keypair.generate();
    // Add your test here.
    let sloValue = new BN(10);
    let sloOperand = { greater: {} };
    let timestampStart = new BN(10000);
    let duration = new BN(1000);
    await program.rpc.initializeSla(
      sloValue,
      sloOperand,
      timestampStart,
      duration,
      {
        accounts: {
          sla: slaKeypair.publicKey,
          owner: owner.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [slaKeypair],
      }
    );

    const pNft = anchor.web3.Keypair.generate();
    const dslaPool = anchor.web3.Keypair.generate();
    const mint = anchor.web3.Keypair.generate();
    const tokenProgram = anchor.web3.Keypair.generate();
    const tokenMetadataProgram = anchor.web3.Keypair.generate();
    const rent = anchor.web3.Keypair.generate();
    const metadata = anchor.web3.Keypair.generate();

    let amount = new BN(1000000000);
    let position = { long: {} };
    await await program.rpc.stake(amount, position, {
      accounts: {
        payer: owner.publicKey,
        pNft: pNft.publicKey,
        dslaPool: dslaPool.publicKey,
        mint: mint.publicKey,
        tokenProgram: tokenProgram.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenMetadataProgram: tokenMetadataProgram.publicKey,
        rent: rent.publicKey,
        metadata: metadata.publicKey,
      },
      signers: [owner],
    });
  });
});
