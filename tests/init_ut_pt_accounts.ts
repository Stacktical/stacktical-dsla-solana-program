import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import {
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintToChecked,
} from "@solana/spl-token";

describe("Initialize UT, PT accounts", () => {
  const PERIOD_REGISTRY: string = "period-registry";
  const PROVIDER_POOL: string = "provider-vault";
  const USER_POOL: string = "user-vault";
  const UT_MINT: string = "ut-mint";
  const PT_MINT: string = "pt-mint";
  const UT_ACCOUNT: string = "ut-account";
  const PT_ACCOUNT: string = "pt-account";
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;
  const slaRegistryKeypair = anchor.web3.Keypair.generate();

  const deployer = Keypair.generate();
  const staker = Keypair.generate();
  let stakerTokenAccount;

  const space = 10_000_000;
  const slaKeypairs = [
    Keypair.generate(),
    Keypair.generate(),
    Keypair.generate(),
  ];

  let mint = null;

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

    const createAccountTransaction = new Transaction().add(
      SystemProgram.createAccount(createAccountParams)
    );

    await sendAndConfirmTransaction(connection, createAccountTransaction, [
      deployer,
      slaRegistryKeypair,
    ]);

    await program.methods
      .initSlaRegistry()
      .accounts({
        deployer: deployer.publicKey,
        slaRegistry: slaRegistryKeypair.publicKey,
      })
      .signers([deployer])
      .rpc();

    mint = await createMint(
      provider.connection,
      deployer,
      deployer.publicKey,
      null,
      0,
      Keypair.generate(),
      {},
      TOKEN_PROGRAM_ID
    );

    stakerTokenAccount = await createAssociatedTokenAccount(
      provider.connection, // connection
      staker, // fee payer
      mint, // mint
      staker.publicKey // owner,
    );

    await mintToChecked(
      provider.connection, // connection
      deployer, // fee payer
      mint, // Mint for the account
      stakerTokenAccount, // receiver (sholud be a token account)
      deployer, // mint authority
      LAMPORTS_PER_SOL * 100, // amount
      0 // decimals
    );
  });

  it("initializes the UT and PT accounts", async () => {
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

    const [periodRegistryPda, _periodRegistryBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(PERIOD_REGISTRY),
          slaKeypairs[0].publicKey.toBuffer(),
        ],
        program.programId
      );

    const [userPoolPda, _userPoolBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(USER_POOL),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [providerPoolPda, _providerPoolBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(PROVIDER_POOL),
          slaKeypairs[0].publicKey.toBuffer(),
        ],
        program.programId
      );

    const [utMintPda, _utMintBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(UT_MINT),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [ptMintPda, _ptMintBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(PT_MINT),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [utAccountPda, _utAccountBump] = await PublicKey.findProgramAddress(
      [
        staker.publicKey.toBuffer(),
        anchor.utils.bytes.utf8.encode(UT_ACCOUNT),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [ptAccountPda, _ptAccountBump] = await PublicKey.findProgramAddress(
      [
        staker.publicKey.toBuffer(),
        anchor.utils.bytes.utf8.encode(PT_ACCOUNT),
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
        .deploySla(ipfsHash, slo, messengerAddress, periods, leverage)
        .accounts({
          deployer: deployer.publicKey,
          slaRegistry: slaRegistryKeypair.publicKey,
          sla: slaKeypairs[0].publicKey,
          slaAuthority: slaAuthorityPda,
          periodRegistry: periodRegistryPda,
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

    try {
      await program.methods
        .initUtPtAccounts()
        .accounts({
          signer: staker.publicKey,
          sla: slaKeypairs[0].publicKey,
          stakerUtAccount: utAccountPda,
          stakerPtAccount: ptAccountPda,
          utMint: utMintPda,
          ptMint: ptMintPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([staker])
        .rpc();
    } catch (err) {
      console.log(err);
    }
  });
});
