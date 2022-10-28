import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import {
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import * as splToken from "@solana/spl-token";

import {
  SLA_REGISTRY_KEYPAIR,
  STATUS_REGISTRY_SEED,
  PROVIDER_POOL_SEED,
  PT_MINT_SEED,
  USER_POOL_SEED,
  UT_MINT_SEED,
} from "./constants";

import { SwitchboardTestContext } from "@switchboard-xyz/sbv2-utils";
import { AggregatorAccount } from "@switchboard-xyz/switchboard-v2";

describe("Validate", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;

  const deployer = Keypair.generate();
  const staker = Keypair.generate();
  let stakerTokenAccount;

  const slaKeypairs = [
    Keypair.generate(),
    Keypair.generate(),
    Keypair.generate(),
  ];

  let mint = null;

  before(async function () {
    let airdropSignature1 = await connection.requestAirdrop(
      deployer.publicKey,
      LAMPORTS_PER_SOL * 1000
    );
    await connection.confirmTransaction(airdropSignature1);

    let airdropSignature2 = await connection.requestAirdrop(
      staker.publicKey,
      LAMPORTS_PER_SOL * 1000
    );
    await connection.confirmTransaction(airdropSignature2);

    mint = await splToken.createMint(
      provider.connection,
      deployer,
      deployer.publicKey,
      null,
      0,
      Keypair.generate(),
      {},
      splToken.TOKEN_PROGRAM_ID
    );

    stakerTokenAccount = await splToken.createAssociatedTokenAccount(
      provider.connection, // connection
      staker, // fee payer
      mint, // mint
      staker.publicKey // owner,
    );

    await splToken.mintToChecked(
      provider.connection, // connection
      deployer, // fee payer
      mint, // Mint for the account
      stakerTokenAccount, // receiver (sholud be a token account)
      deployer, // mint authority
      LAMPORTS_PER_SOL * 100, // amount
      0 // decimals
    );
  });

  it("stakes provider side", async () => {
    const ipfsHash = "t";
    const sloType = { greaterThan: {} };
    let sloValue = {
      mantissa: new anchor.BN("100"),
      scale: new anchor.BN("0"),
    };
    const slo = {
      sloValue,
      sloType,
    };

    const messengerAddress = anchor.web3.Keypair.generate().publicKey;
    const periods = [
      {
        start: new anchor.BN("1000000"),
        end: new anchor.BN("1900000"),
        status: { notVerified: {} },
      },
    ];
    const leverage = new anchor.BN("1");

    const [statusRegistryPda, _statusRegistryBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(STATUS_REGISTRY_SEED),
          slaKeypairs[0].publicKey.toBuffer(),
        ],
        program.programId
      );

    const [userPoolPda, _userPoolBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(USER_POOL_SEED),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [providerPoolPda, _providerPoolBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(PROVIDER_POOL_SEED),
          slaKeypairs[0].publicKey.toBuffer(),
        ],
        program.programId
      );

    const [utMintPda, _utMintBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(UT_MINT_SEED),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [ptMintPda, _ptMintBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(PT_MINT_SEED),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [slaAuthorityPda, _slaAuthorityBump] =
      await PublicKey.findProgramAddress(
        [slaKeypairs[0].publicKey.toBuffer()],
        program.programId
      );

    try {
      await program.methods
        .deploySla(ipfsHash, slo, messengerAddress, leverage)
        .accounts({
          deployer: deployer.publicKey,
          slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
          sla: slaKeypairs[0].publicKey,
          slaAuthority: slaAuthorityPda,
          statusRegistry: statusRegistryPda,
          mint: mint,
          providerPool: providerPoolPda,
          userPool: userPoolPda,
          utMint: utMintPda,
          ptMint: ptMintPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([deployer, slaKeypairs[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    // load the Switchboard env to dictate which queue to create feed for
    const switchboard = await SwitchboardTestContext.loadFromEnv(
      anchor.AnchorProvider.env()
    );

    // create a static feed that will always resolve to 100
    // then call openRound and wait for the oracle to process the update
    const aggregatorAccount: AggregatorAccount =
      await switchboard.createStaticFeed(100);

    await program.methods.validatePeriod(new anchor.BN("0")).accounts({
      validator: deployer.publicKey,
      sla: slaKeypairs[0].publicKey,
      statusRegistry: statusRegistryPda,
      aggregator: aggregatorAccount.publicKey,
    });
  });
});
