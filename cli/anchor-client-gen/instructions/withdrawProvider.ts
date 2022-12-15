import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface WithdrawProviderArgs {
  tokenAmount: BN
}

export interface WithdrawProviderAccounts {
  /** provider */
  withdrawer: PublicKey
  /** the SLA */
  sla: PublicKey
  slaAuthority: PublicKey
  /** The token account to claimer the money in */
  withdrawerTokenAccount: PublicKey
  /** The token account with pt tokens */
  withdrawerPtAccount: PublicKey
  ptLockup: PublicKey
  mint: PublicKey
  pool: PublicKey
  ptMint: PublicKey
  governance: PublicKey
  tokenProgram: PublicKey
  program: PublicKey
  programData: PublicKey
  protocolTokenAccount: PublicKey
  deployerTokenAccount: PublicKey
  rent: PublicKey
  systemProgram: PublicKey
}

export const layout = borsh.struct([borsh.u64("tokenAmount")])

export function withdrawProvider(
  args: WithdrawProviderArgs,
  accounts: WithdrawProviderAccounts
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
    { pubkey: accounts.withdrawerPtAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.ptLockup, isSigner: false, isWritable: true },
    { pubkey: accounts.mint, isSigner: false, isWritable: true },
    { pubkey: accounts.pool, isSigner: false, isWritable: true },
    { pubkey: accounts.ptMint, isSigner: false, isWritable: true },
    { pubkey: accounts.governance, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.program, isSigner: false, isWritable: false },
    { pubkey: accounts.programData, isSigner: false, isWritable: false },
    {
      pubkey: accounts.protocolTokenAccount,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: accounts.deployerTokenAccount,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([122, 6, 188, 45, 22, 219, 125, 99])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      tokenAmount: args.tokenAmount,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
