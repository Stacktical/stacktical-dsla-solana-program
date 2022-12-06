import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface InitLockupAccountsAccounts {
  userProvider: PublicKey
  sla: PublicKey
  ptLockup: PublicKey
  utLockup: PublicKey
  systemProgram: PublicKey
}

export function initLockupAccounts(accounts: InitLockupAccountsAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.userProvider, isSigner: true, isWritable: true },
    { pubkey: accounts.sla, isSigner: false, isWritable: false },
    { pubkey: accounts.ptLockup, isSigner: false, isWritable: true },
    { pubkey: accounts.utLockup, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([241, 139, 234, 6, 16, 68, 244, 86])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
