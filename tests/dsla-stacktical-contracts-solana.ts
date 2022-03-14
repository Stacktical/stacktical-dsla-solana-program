import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";

import {
  PublicKey,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintToChecked,
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";

import { StackticalDslaContractsSolana } from "../target/types/stacktical_dsla_contracts_solana";

describe("stacktical-dsla-contracts-solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace
    .StackticalDslaContractsSolana as Program<StackticalDslaContractsSolana>;

  let mintCustomPubkey = null as PublicKey;
  let mintPFTPubkey = null as PublicKey;

  let initializerTokenAccountCustomPubkey = null;
  let initializerTokenAccountPFTPubkey = null;

  let user1TokenAccountCustomPubkey = null;
  let user1TokenAccountPFTPubkey = null;

  let user2TokenAccountCustomPubkey = null;
  let user2TokenAccountPFTPubkey = null;

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

  before(async () => {
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

    mintCustomPubkey = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      18,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    mintPFTPubkey = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      18,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    // initializer
    initializerTokenAccountCustomPubkey = await createAssociatedTokenAccount(
      provider.connection,
      initializerMainAccount,
      mintCustomPubkey,
      initializerMainAccount.publicKey
    );
    initializerTokenAccountPFTPubkey = await createAssociatedTokenAccount(
      provider.connection,
      initializerMainAccount,
      mintPFTPubkey,
      initializerMainAccount.publicKey
    );

    // user 1
    user1TokenAccountCustomPubkey = await createAssociatedTokenAccount(
      provider.connection,
      user1MainAccount,
      mintCustomPubkey,
      user1MainAccount.publicKey
    );
    user1TokenAccountPFTPubkey = await createAssociatedTokenAccount(
      provider.connection,
      user1MainAccount,
      mintPFTPubkey,
      user1MainAccount.publicKey
    );

    // user 2
    user2TokenAccountCustomPubkey = await createAssociatedTokenAccount(
      provider.connection,
      user2MainAccount,
      mintCustomPubkey,
      user2MainAccount.publicKey
    );
    user2TokenAccountPFTPubkey = await createAssociatedTokenAccount(
      provider.connection,
      user2MainAccount,
      mintPFTPubkey,
      user2MainAccount.publicKey
    );
  });
  it("Account are initialized correctly!", async () => {
    await mintToChecked(
      provider.connection,
      initializerMainAccount,
      mintCustomPubkey,
      initializerTokenAccountCustomPubkey,
      mintAuthority,
      initializerAmount,
      18
    );

    let tokenAmount = await provider.connection.getTokenAccountBalance(
      initializerTokenAccountCustomPubkey
    );

    expect(Number(tokenAmount.value.amount)).to.equal(initializerAmount);
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

    console.log(vault_authority_pda);
    await program.rpc.initialize(
      sloValue,
      sloOperand,
      timestampStart,
      duration,
      new BN(initializerAmount),
      {
        accounts: {
          initializer: initializerMainAccount.publicKey,
          mint: mintCustomPubkey,
          vaultAccount: vault_account_pda,
          initializerDepositTokenAccount: initializerTokenAccountCustomPubkey,
          sla: sla.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        instructions: [await program.account.sla.createInstruction(sla)],
        signers: [sla, initializerMainAccount],
      }
    );

    let _vault = await getAccount(provider.connection, vault_account_pda);

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
