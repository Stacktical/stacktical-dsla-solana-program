import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId";

export interface WithdrawUserArgs {
  tokenAmount: BN;
}

export interface WithdrawUserAccounts {
  /** user */
  withdrawer: PublicKey;
  /** the SLA */
  sla: PublicKey;
  slaAuthority: PublicKey;
  /** The token account to claimer the money in */
  withdrawerTokenAccount: PublicKey;
  /** The token account with ut tokens */
  withdrawerUtAccount: PublicKey;
  mint: PublicKey;
  pool: PublicKey;
  utMint: PublicKey;
  utLockup: PublicKey;
  deployerTokenAccount: PublicKey;
  protocolTokenAccount: PublicKey;
  governance: PublicKey;
  tokenProgram: PublicKey;
  program: PublicKey;
  programData: PublicKey;
  rent: PublicKey;
  systemProgram: PublicKey;
}

export const layout = borsh.struct([borsh.u64("tokenAmount")]);

export function withdrawUser(
  args: WithdrawUserArgs,
  accounts: WithdrawUserAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.withdrawer, isSigner: true, isWritable: true },
    { pubkey: accounts.sla, isSigner: false, isWritable: true },
    { pubkey: accounts.slaAuthority, isSigner: false, isWritable: true },
    {
      pubkey: accounts.withdrawerTokenAccount,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.withdrawerUtAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.mint, isSigner: false, isWritable: false },
    { pubkey: accounts.pool, isSigner: false, isWritable: true },
    { pubkey: accounts.utMint, isSigner: false, isWritable: false },
    { pubkey: accounts.utLockup, isSigner: false, isWritable: false },
    {
      pubkey: accounts.deployerTokenAccount,
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: accounts.protocolTokenAccount,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.governance, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.program, isSigner: false, isWritable: false },
    { pubkey: accounts.programData, isSigner: false, isWritable: false },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ];
  const identifier = Buffer.from([86, 169, 152, 107, 33, 180, 134, 115]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      tokenAmount: args.tokenAmount,
    },
    buffer
  );
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len);
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data });
  return ix;
}
