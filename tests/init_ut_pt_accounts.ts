import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Dsla } from "../target/types/dsla";
import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";

import {
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import { NATIVE_MINT } from "@solana/spl-token";
import {
  PERIOD_REGISTRY_SEED,
  PROVIDER_POOL_SEED,
  PT_ACCOUNT_SEED,
  PT_MINT_SEED,
  SLA_REGISTRY_KEYPAIR,
  USER_POOL_SEED,
  UT_ACCOUNT_SEED,
  UT_MINT_SEED,
} from "./constants";

describe("Initialize UT, PT accounts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;

  const slaKeypairs = [Keypair.generate()];

  const deployer = Keypair.generate();
  const staker = Keypair.generate();

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
          anchor.utils.bytes.utf8.encode(PERIOD_REGISTRY_SEED),
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

    const [utAccountPda, _utAccountBump] = await PublicKey.findProgramAddress(
      [
        staker.publicKey.toBuffer(),
        anchor.utils.bytes.utf8.encode(UT_ACCOUNT_SEED),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [ptAccountPda, _ptAccountBump] = await PublicKey.findProgramAddress(
      [
        staker.publicKey.toBuffer(),
        anchor.utils.bytes.utf8.encode(PT_ACCOUNT_SEED),
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
          slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
          sla: slaKeypairs[0].publicKey,
          slaAuthority: slaAuthorityPda,
          periodRegistry: periodRegistryPda,
          mint: NATIVE_MINT,
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
