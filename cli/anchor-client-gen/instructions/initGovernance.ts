import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface InitGovernanceArgs {
  dslaDepositByPeriod: BN
  dslaProtocolReward: BN
  dslaValidatorReward: BN
  dslaBurnedByVerification: BN
  slaDeployerRewardsRate: types.DslaDecimalFields
  protocolRewardsRate: types.DslaDecimalFields
  maxLeverage: types.DslaDecimalFields
}

export interface InitGovernanceAccounts {
  /** the account that has the authority to upgrade the program */
  programUpgradeAuthority: PublicKey
  governance: PublicKey
  program: PublicKey
  programData: PublicKey
  systemProgram: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("dslaDepositByPeriod"),
  borsh.u64("dslaProtocolReward"),
  borsh.u64("dslaValidatorReward"),
  borsh.u64("dslaBurnedByVerification"),
  types.DslaDecimal.layout("slaDeployerRewardsRate"),
  types.DslaDecimal.layout("protocolRewardsRate"),
  types.DslaDecimal.layout("maxLeverage"),
])

export function initGovernance(
  args: InitGovernanceArgs,
  accounts: InitGovernanceAccounts
) {
  const keys: Array<AccountMeta> = [
    {
      pubkey: accounts.programUpgradeAuthority,
      isSigner: true,
      isWritable: true,
    },
    { pubkey: accounts.governance, isSigner: false, isWritable: true },
    { pubkey: accounts.program, isSigner: false, isWritable: false },
    { pubkey: accounts.programData, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([23, 241, 166, 67, 20, 30, 182, 32])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      dslaDepositByPeriod: args.dslaDepositByPeriod,
      dslaProtocolReward: args.dslaProtocolReward,
      dslaValidatorReward: args.dslaValidatorReward,
      dslaBurnedByVerification: args.dslaBurnedByVerification,
      slaDeployerRewardsRate: types.DslaDecimal.toEncodable(
        args.slaDeployerRewardsRate
      ),
      protocolRewardsRate: types.DslaDecimal.toEncodable(
        args.protocolRewardsRate
      ),
      maxLeverage: types.DslaDecimal.toEncodable(args.maxLeverage),
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
