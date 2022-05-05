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
import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";

describe("Deploy SLA", () => {
  const PERIOD_REGISTRY: string = "period-registry";
  const PROVIDER_VAULT: string = "provider-vault";
  const USER_VAULT: string = "user-vault";
  const UT_MINT: string = "ut-mint";
  const PT_MINT: string = "pt-mint";
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.Dsla as Program<Dsla>;
  const slaRegistryKeypair = anchor.web3.Keypair.generate();

  const deployer = Keypair.generate();

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
  });

  it("Deploys an SLA 1", async () => {
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

    const [userVaultPda, _userVaultBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(USER_VAULT),
        slaKeypairs[0].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [providerVaultPda, _providerVaultBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(PROVIDER_VAULT),
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

    try {
      await program.methods
        .deploySla(ipfsHash, slo, messengerAddress, periods, leverage)
        .accounts({
          deployer: deployer.publicKey,
          slaRegistry: slaRegistryKeypair.publicKey,
          sla: slaKeypairs[0].publicKey,
          periodRegistry: periodRegistryPda,
          mint: mint,
          providerVault: providerVaultPda,
          userVault: userVaultPda,
          utMint: utMintPda,
          ptMint: ptMintPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([deployer, slaKeypairs[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    const expectedSlaAccountAddresses = [slaKeypairs[0].publicKey];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(slaRegistryKeypair.publicKey)
    ).slaAccountAddresses;

    expect(
      actualSlaAccountAddresses[0].toString(),
      "SLA registry address doesn't match  the expected address"
    ).to.equal(expectedSlaAccountAddresses[0].toString());

    expect(
      actualSlaAccountAddresses.length,
      "SLA registry lenghth doesn't match"
    ).to.equal(expectedSlaAccountAddresses.length);

    expect(
      actualSlaAccountAddresses[0].toString(),
      "match to wrong address"
    ).to.not.equal(slaKeypairs[1].publicKey.toString());
  });

  it("Deploys an SLA 2", async () => {
    const ipfsHash = "tt";
    let sloType = { smallerThan: {} };
    const slo = { sloValue: new anchor.BN("999"), sloType };
    const messengerAddress = anchor.web3.Keypair.generate().publicKey;
    const periods = [
      {
        start: new anchor.BN("99999999"),
        end: new anchor.BN("10000000000"),
        status: { notVerified: {} },
      },
    ];
    const leverage = new anchor.BN("5");

    const [periodRegistryPda, _periodRegistryBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(PERIOD_REGISTRY),
          slaKeypairs[1].publicKey.toBuffer(),
        ],
        program.programId
      );

    const [userVaultPda, _userVaultBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(USER_VAULT),
        slaKeypairs[1].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [providerVaultPda, _providerVaultBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(PROVIDER_VAULT),
          slaKeypairs[1].publicKey.toBuffer(),
        ],
        program.programId
      );

    const [utMintPda, _utMintBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(UT_MINT),
        slaKeypairs[1].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [ptMintPda, _ptMintBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(PT_MINT),
        slaKeypairs[1].publicKey.toBuffer(),
      ],
      program.programId
    );

    try {
      await program.methods
        .deploySla(ipfsHash, slo, messengerAddress, periods, leverage)
        .accounts({
          deployer: deployer.publicKey,
          slaRegistry: slaRegistryKeypair.publicKey,
          sla: slaKeypairs[1].publicKey,
          periodRegistry: periodRegistryPda,
          mint: mint,
          providerVault: providerVaultPda,
          userVault: userVaultPda,
          utMint: utMintPda,
          ptMint: ptMintPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([deployer, slaKeypairs[1]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    const expectedSlaAccountAddresses = [
      slaKeypairs[0].publicKey,
      slaKeypairs[1].publicKey,
    ];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(slaRegistryKeypair.publicKey)
    ).slaAccountAddresses;

    expect(
      actualSlaAccountAddresses[0].toString(),
      "SLA registry address doesn't match  the expected address"
    ).to.equal(expectedSlaAccountAddresses[0].toString());

    expect(
      actualSlaAccountAddresses[1].toString(),
      "SLA registry address doesn't match  the expected address"
    ).to.equal(expectedSlaAccountAddresses[1].toString());

    expect(
      actualSlaAccountAddresses.length,
      "SLA registry lenghth doesn't match"
    ).to.equal(expectedSlaAccountAddresses.length);
  });
});
