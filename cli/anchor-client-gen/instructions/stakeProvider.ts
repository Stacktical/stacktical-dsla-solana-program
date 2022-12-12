import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId";

export interface StakeProviderArgs {
  tokenAmount: BN;
}

export interface StakeProviderAccounts {
  staker: PublicKey;
  sla: PublicKey;
  slaAuthority: PublicKey;
  mint: PublicKey;
  pool: PublicKey;
  ptMint: PublicKey;
  /** The account to claim the money from */
  stakerTokenAccount: PublicKey;
  /** pt tokens */
  stakerPtAccount: PublicKey;
  ptLockup: PublicKey;
  tokenProgram: PublicKey;
  rent: PublicKey;
  systemProgram: PublicKey;
}

export const layout = borsh.struct([borsh.u64("tokenAmount")]);

export function stakeProvider(
  args: StakeProviderArgs,
  accounts: StakeProviderAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.staker, isSigner: true, isWritable: true },
    { pubkey: accounts.sla, isSigner: false, isWritable: true },
    { pubkey: accounts.slaAuthority, isSigner: false, isWritable: true },
    { pubkey: accounts.mint, isSigner: false, isWritable: false },
    { pubkey: accounts.pool, isSigner: false, isWritable: true },
    { pubkey: accounts.ptMint, isSigner: false, isWritable: true },
    { pubkey: accounts.stakerTokenAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.stakerPtAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.ptLockup, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ];
  const identifier = Buffer.from([18, 199, 109, 78, 14, 224, 5, 119]);
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
