import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";

import {
  PublicKey,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { expect } from "chai";

import { StackticalDslaContractsSolana } from "../target/types/stacktical_dsla_contracts_solana";

describe("stacktical-dsla-contracts-solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace
    .StackticalDslaContractsSolana as Program<StackticalDslaContractsSolana>;

  let mintDSLA = null as Token;
  let mintPFT = null as Token;

  let initializerTokenAccountDSLA = null;
  let initializerTokenAccountPFT = null;

  let user1TokenAccountDSLA = null;
  let user1TokenAccountPFT = null;

  let user2TokenAccountDSLA = null;
  let user2TokenAccountPFT = null;

  let vault_account_pda = null;
  let vault_account_bump = null;
  let vault_authority_pda = null;

  const funding = LAMPORTS_PER_SOL * 10000;

  const sloValue = new anchor.BN(7);
  const sloOperand = { greater: {} };
  const timestampStart = new anchor.BN(1897651197);
  const duration = new anchor.BN(5000000);
  const initializerAmount = LAMPORTS_PER_SOL * 100;

  const sla = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();
  const mintAuthority = anchor.web3.Keypair.generate();
  const initializerMainAccount = anchor.web3.Keypair.generate();
  const user1MainAccount = anchor.web3.Keypair.generate();
  const user2MainAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, funding),
      "processed"
    );

    // Fund Main Accounts
    await provider.send(
      (() => {
        const tx = new Transaction();
        tx.add(
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: initializerMainAccount.publicKey,
            lamports: funding / 10,
          }),
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: user1MainAccount.publicKey,
            lamports: funding / 10,
          }),
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: user2MainAccount.publicKey,
            lamports: funding / 10,
          })
        );
        return tx;
      })(),
      [payer]
    );

    mintDSLA = await Token.createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      18,
      TOKEN_PROGRAM_ID
    );

    mintPFT = await Token.createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      18,
      TOKEN_PROGRAM_ID
    );

    // initializer
    initializerTokenAccountDSLA = await mintDSLA.createAccount(
      initializerMainAccount.publicKey
    );
    initializerTokenAccountPFT = await mintPFT.createAccount(
      initializerMainAccount.publicKey
    );

    // user 1
    user1TokenAccountDSLA = await mintDSLA.createAccount(
      user1MainAccount.publicKey
    );
    user1TokenAccountPFT = await mintPFT.createAccount(
      user1MainAccount.publicKey
    );

    // user 2
    user2TokenAccountDSLA = await mintPFT.createAccount(
      user2MainAccount.publicKey
    );
    user2TokenAccountPFT = await mintPFT.createAccount(
      user2MainAccount.publicKey
    );

    await mintDSLA.mintTo(
      initializerTokenAccountDSLA,
      mintAuthority.publicKey,
      [mintAuthority],
      initializerAmount
    );

    let _initializerTokenAccountDSLA = await mintDSLA.getAccountInfo(
      initializerTokenAccountDSLA
    );

    expect(_initializerTokenAccountDSLA.amount.toNumber()).to.equal(
      initializerAmount
    );
  });

  it("Initialize SLA", async () => {
    const [_vault_account_pda, _vault_account_bump] =
      await PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("token-seed"))],
        program.programId
      );
    vault_account_pda = _vault_account_pda;
    vault_account_bump = _vault_account_bump;

    const [_vault_authority_pda, _vault_authority_bump] =
      await PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("sla"))],
        program.programId
      );
    vault_authority_pda = _vault_authority_pda;

    await program.rpc.initialize(
      sloValue,
      sloOperand,
      timestampStart,
      duration,
      new BN(initializerAmount),
      {
        accounts: {
          initializer: initializerMainAccount.publicKey,
          mint: mintDSLA.publicKey,
          vaultAccount: vault_account_pda,
          initializerDepositTokenAccount: initializerTokenAccountDSLA,
          sla: sla.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        instructions: [await program.account.sla.createInstruction(sla)],
        signers: [sla, initializerMainAccount],
      }
    );

    let _vault = await mintDSLA.getAccountInfo(vault_account_pda);

    let _sla = await program.account.sla.fetch(sla.publicKey);

    // Check that the new owner is the PDA.
    expect(_vault.owner).to.equals(vault_authority_pda);

    // Check that the values in the sla account match what we expect.
    expect(_sla.sloValue).to.equals(sloValue);
    expect(_sla.sloOperand).to.equals(sloOperand);
    expect(_sla.breached).to.equals(false);
    expect(_sla.timestampStart).to.equals(timestampStart);
    expect(_sla.duration).to.equals(duration);
  });
});
