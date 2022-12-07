import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId";

export interface StakeUserArgs {
  tokenAmount: BN;
}

export interface StakeUserAccounts {
  staker: PublicKey;
  sla: PublicKey;
  slaAuthority: PublicKey;
  mint: PublicKey;
  pool: PublicKey;
  utMint: PublicKey;
  utLockup: PublicKey;
  /** The account to claim the money from */
  stakerTokenAccount: PublicKey;
  /** ut tokens */
  stakerUtAccount: PublicKey;
  tokenProgram: PublicKey;
  rent: PublicKey;
  systemProgram: PublicKey;
}

export const layout = borsh.struct([borsh.u64("tokenAmount")]);

export function stakeUser(args: StakeUserArgs, accounts: StakeUserAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.staker, isSigner: true, isWritable: true },
    { pubkey: accounts.sla, isSigner: false, isWritable: true },
    { pubkey: accounts.slaAuthority, isSigner: false, isWritable: true },
    { pubkey: accounts.mint, isSigner: false, isWritable: false },
    { pubkey: accounts.pool, isSigner: false, isWritable: true },
    { pubkey: accounts.utMint, isSigner: false, isWritable: true },
    { pubkey: accounts.utLockup, isSigner: false, isWritable: true },
    { pubkey: accounts.stakerTokenAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.stakerUtAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ];
  const identifier = Buffer.from([145, 223, 129, 230, 185, 115, 48, 18]);
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
