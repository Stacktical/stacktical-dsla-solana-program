import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface ValidatePeriodArgs {
  period: BN
}

export interface ValidatePeriodAccounts {
  validator: PublicKey
  slaAuthority: PublicKey
  statusRegistry: PublicKey
  sla: PublicKey
  aggregator: PublicKey
  governance: PublicKey
  dslaMint: PublicKey
  dslaPool: PublicKey
  /** The validator token account to pay the DSLA reward to */
  validatorDslaTokenAccount: PublicKey
  program: PublicKey
  programData: PublicKey
  protocol: PublicKey
  protocolTokenAccount: PublicKey
  /** The program for interacting with the token. */
  tokenProgram: PublicKey
  rent: PublicKey
  systemProgram: PublicKey
}

export const layout = borsh.struct([borsh.u64("period")])

export function validatePeriod(
  args: ValidatePeriodArgs,
  accounts: ValidatePeriodAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.validator, isSigner: true, isWritable: true },
    { pubkey: accounts.slaAuthority, isSigner: false, isWritable: true },
    { pubkey: accounts.statusRegistry, isSigner: false, isWritable: true },
    { pubkey: accounts.sla, isSigner: false, isWritable: true },
    { pubkey: accounts.aggregator, isSigner: false, isWritable: false },
    { pubkey: accounts.governance, isSigner: false, isWritable: false },
    { pubkey: accounts.dslaMint, isSigner: false, isWritable: false },
    { pubkey: accounts.dslaPool, isSigner: false, isWritable: false },
    {
      pubkey: accounts.validatorDslaTokenAccount,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.program, isSigner: false, isWritable: false },
    { pubkey: accounts.programData, isSigner: false, isWritable: false },
    { pubkey: accounts.protocol, isSigner: false, isWritable: false },
    {
      pubkey: accounts.protocolTokenAccount,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([204, 243, 114, 76, 3, 131, 47, 171])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      period: args.period,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
