import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId";

export interface DeploySlaArgs {
  slo: types.SloFields;
  leverage: types.DslaDecimalFields;
  start: BN;
  nPeriods: number;
  periodLength: types.PeriodLengthKind;
}

export interface DeploySlaAccounts {
  deployer: PublicKey;
  slaRegistry: PublicKey;
  sla: PublicKey;
  slaAuthority: PublicKey;
  statusRegistry: PublicKey;
  mint: PublicKey;
  pool: PublicKey;
  dslaMint: PublicKey;
  dslaPool: PublicKey;
  /** The token account to pay the DSLA fee from */
  deployerDslaTokenAccount: PublicKey;
  governance: PublicKey;
  utMint: PublicKey;
  ptMint: PublicKey;
  aggregator: PublicKey;
  /** The program for interacting with the token. */
  tokenProgram: PublicKey;
  rent: PublicKey;
  systemProgram: PublicKey;
}

export const layout = borsh.struct([
  types.Slo.layout("slo"),
  types.DslaDecimal.layout("leverage"),
  borsh.u128("start"),
  borsh.u32("nPeriods"),
  types.PeriodLength.layout("periodLength"),
]);

export function deploySla(args: DeploySlaArgs, accounts: DeploySlaAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.deployer, isSigner: true, isWritable: true },
    { pubkey: accounts.slaRegistry, isSigner: false, isWritable: true },
    { pubkey: accounts.sla, isSigner: true, isWritable: true },
    { pubkey: accounts.slaAuthority, isSigner: false, isWritable: true },
    { pubkey: accounts.statusRegistry, isSigner: false, isWritable: true },
    { pubkey: accounts.mint, isSigner: false, isWritable: false },
    { pubkey: accounts.pool, isSigner: false, isWritable: true },
    { pubkey: accounts.dslaMint, isSigner: false, isWritable: false },
    { pubkey: accounts.dslaPool, isSigner: false, isWritable: true },
    {
      pubkey: accounts.deployerDslaTokenAccount,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.governance, isSigner: false, isWritable: false },
    { pubkey: accounts.utMint, isSigner: false, isWritable: true },
    { pubkey: accounts.ptMint, isSigner: false, isWritable: true },
    { pubkey: accounts.aggregator, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ];
  const identifier = Buffer.from([147, 228, 145, 146, 170, 51, 48, 158]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      slo: types.Slo.toEncodable(args.slo),
      leverage: types.DslaDecimal.toEncodable(args.leverage),
      start: args.start,
      nPeriods: args.nPeriods,
      periodLength: args.periodLength.toEncodable(),
    },
    buffer
  );
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len);
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data });
  return ix;
}
