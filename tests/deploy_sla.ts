import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import { SystemProgram, Keypair, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, NATIVE_MINT } from "@solana/spl-token";
import {
  DEPLOYER,
  STATUS_REGISTRY_SEED,
  POOL_SEED,
  PT_MINT_SEED,
  SLA_REGISTRY_KEYPAIR,
  UT_MINT_SEED,
} from "./constants";
describe("Deploy SLA", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.Dsla as Program<Dsla>;

  const slaKeypairs = [
    Keypair.generate(),
    Keypair.generate(),
    Keypair.generate(),
  ];

  it("Deploys an SLA 1", async () => {
    let start = new anchor.BN("7000000");
    let n_periods = new anchor.BN("100");
    let period_length = { custom: { lenght: 128 } };

    let sloType = { greaterThan: {} };
    let sloValue = {
      mantissa: new anchor.BN("100"),
      scale: new anchor.BN("0"),
    };
    const slo = {
      sloValue,
      sloType,
    };
    const messengerAddress = anchor.web3.Keypair.generate().publicKey;

    const leverage = new anchor.BN("1");

    const [statusRegistryPda, _statusRegistryBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(STATUS_REGISTRY_SEED),
          slaKeypairs[0].publicKey.toBuffer(),
        ],
        program.programId
      );

    const [poolPda, PoolBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(POOL_SEED),
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
        .deploySla(
          slo,
          messengerAddress,
          leverage,
          start,
          n_periods,
          period_length
        )
        .accounts({
          deployer: DEPLOYER.publicKey,
          slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
          sla: slaKeypairs[0].publicKey,
          slaAuthority: slaAuthorityPda,
          mint: NATIVE_MINT,
          statusRegistry: statusRegistryPda,
          pool: poolPda,
          utMint: utMintPda,
          ptMint: ptMintPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([DEPLOYER, slaKeypairs[0]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    const expectedSlaAccountAddresses = [slaKeypairs[0].publicKey];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(SLA_REGISTRY_KEYPAIR.publicKey)
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
    let start = new anchor.BN("10000000");
    let n_periods = new anchor.BN("5");
    let period_length = { custom: { lenght: 10000 } };

    let sloType = { smallerThan: {} };
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
        start: new anchor.BN("99999999"),
        end: new anchor.BN("10000000000"),
        status: { notVerified: {} },
      },
    ];
    const leverage = new anchor.BN("5");

    const [statusRegistryPda, _statusRegistryBump] =
      await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(STATUS_REGISTRY_SEED),
          slaKeypairs[1].publicKey.toBuffer(),
        ],
        program.programId
      );

    const [poolPda, _PoolBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(POOL_SEED),
        slaKeypairs[1].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [utMintPda, _utMintBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(UT_MINT_SEED),
        slaKeypairs[1].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [ptMintPda, _ptMintBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(PT_MINT_SEED),
        slaKeypairs[1].publicKey.toBuffer(),
      ],
      program.programId
    );

    const [slaAuthorityPda, _slaAuthorityBump] =
      await PublicKey.findProgramAddress(
        [slaKeypairs[1].publicKey.toBuffer()],
        program.programId
      );

    try {
      await program.methods
        .deploySla(
          slo,
          messengerAddress,
          leverage,
          start,
          n_periods,
          period_length
        )
        .accounts({
          deployer: DEPLOYER.publicKey,
          slaRegistry: SLA_REGISTRY_KEYPAIR.publicKey,
          sla: slaKeypairs[1].publicKey,
          slaAuthority: slaAuthorityPda,
          mint: NATIVE_MINT,
          pool: poolPda,
          statusRegistry: statusRegistryPda,
          utMint: utMintPda,
          ptMint: ptMintPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([DEPLOYER, slaKeypairs[1]])
        .rpc();
    } catch (err) {
      console.log(err);
    }

    const expectedSlaAccountAddresses = [
      slaKeypairs[0].publicKey,
      slaKeypairs[1].publicKey,
    ];
    const actualSlaAccountAddresses = (
      await program.account.slaRegistry.fetch(SLA_REGISTRY_KEYPAIR.publicKey)
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
