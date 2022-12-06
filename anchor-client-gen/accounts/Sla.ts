import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SlaFields {
  /** address of who deployed the SLA */
  slaDeployerAddress: PublicKey
  /** address of the messeger providing the data */
  messengerAddress: PublicKey
  /** address of the switchboard aggregator account */
  aggregatorAddress: PublicKey
  /** service level objective, the objective to achieve for the provider to be rewarded */
  slo: types.SloFields
  /** leverage for the SLA between provider and user pool */
  leverage: types.DslaDecimalFields
  /** address of the coin to be used as SLA reward for users and providers */
  mintAddress: PublicKey
  /** all the data regarding periods. */
  periodData: types.PeriodGeneratorFields
  /** amount of tokens in Provider pool */
  providerPoolSize: BN
  /** amount of tokens in User pool */
  userPoolSize: BN
  /** total user token supply */
  utSupply: BN
  /** total provider token supply */
  ptSupply: BN
}

export interface SlaJSON {
  /** address of who deployed the SLA */
  slaDeployerAddress: string
  /** address of the messeger providing the data */
  messengerAddress: string
  /** address of the switchboard aggregator account */
  aggregatorAddress: string
  /** service level objective, the objective to achieve for the provider to be rewarded */
  slo: types.SloJSON
  /** leverage for the SLA between provider and user pool */
  leverage: types.DslaDecimalJSON
  /** address of the coin to be used as SLA reward for users and providers */
  mintAddress: string
  /** all the data regarding periods. */
  periodData: types.PeriodGeneratorJSON
  /** amount of tokens in Provider pool */
  providerPoolSize: string
  /** amount of tokens in User pool */
  userPoolSize: string
  /** total user token supply */
  utSupply: string
  /** total provider token supply */
  ptSupply: string
}

/** `Sla` is Service level agreement account containing all the variables to make it possible */
export class Sla {
  /** address of who deployed the SLA */
  readonly slaDeployerAddress: PublicKey
  /** address of the messeger providing the data */
  readonly messengerAddress: PublicKey
  /** address of the switchboard aggregator account */
  readonly aggregatorAddress: PublicKey
  /** service level objective, the objective to achieve for the provider to be rewarded */
  readonly slo: types.Slo
  /** leverage for the SLA between provider and user pool */
  readonly leverage: types.DslaDecimal
  /** address of the coin to be used as SLA reward for users and providers */
  readonly mintAddress: PublicKey
  /** all the data regarding periods. */
  readonly periodData: types.PeriodGenerator
  /** amount of tokens in Provider pool */
  readonly providerPoolSize: BN
  /** amount of tokens in User pool */
  readonly userPoolSize: BN
  /** total user token supply */
  readonly utSupply: BN
  /** total provider token supply */
  readonly ptSupply: BN

  static readonly discriminator = Buffer.from([
    93, 177, 43, 102, 221, 228, 221, 169,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("slaDeployerAddress"),
    borsh.publicKey("messengerAddress"),
    borsh.publicKey("aggregatorAddress"),
    types.Slo.layout("slo"),
    types.DslaDecimal.layout("leverage"),
    borsh.publicKey("mintAddress"),
    types.PeriodGenerator.layout("periodData"),
    borsh.u128("providerPoolSize"),
    borsh.u128("userPoolSize"),
    borsh.u128("utSupply"),
    borsh.u128("ptSupply"),
  ])

  constructor(fields: SlaFields) {
    this.slaDeployerAddress = fields.slaDeployerAddress
    this.messengerAddress = fields.messengerAddress
    this.aggregatorAddress = fields.aggregatorAddress
    this.slo = new types.Slo({ ...fields.slo })
    this.leverage = new types.DslaDecimal({ ...fields.leverage })
    this.mintAddress = fields.mintAddress
    this.periodData = new types.PeriodGenerator({ ...fields.periodData })
    this.providerPoolSize = fields.providerPoolSize
    this.userPoolSize = fields.userPoolSize
    this.utSupply = fields.utSupply
    this.ptSupply = fields.ptSupply
  }

  static async fetch(c: Connection, address: PublicKey): Promise<Sla | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(PROGRAM_ID)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[]
  ): Promise<Array<Sla | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(PROGRAM_ID)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): Sla {
    if (!data.slice(0, 8).equals(Sla.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = Sla.layout.decode(data.slice(8))

    return new Sla({
      slaDeployerAddress: dec.slaDeployerAddress,
      messengerAddress: dec.messengerAddress,
      aggregatorAddress: dec.aggregatorAddress,
      slo: types.Slo.fromDecoded(dec.slo),
      leverage: types.DslaDecimal.fromDecoded(dec.leverage),
      mintAddress: dec.mintAddress,
      periodData: types.PeriodGenerator.fromDecoded(dec.periodData),
      providerPoolSize: dec.providerPoolSize,
      userPoolSize: dec.userPoolSize,
      utSupply: dec.utSupply,
      ptSupply: dec.ptSupply,
    })
  }

  toJSON(): SlaJSON {
    return {
      slaDeployerAddress: this.slaDeployerAddress.toString(),
      messengerAddress: this.messengerAddress.toString(),
      aggregatorAddress: this.aggregatorAddress.toString(),
      slo: this.slo.toJSON(),
      leverage: this.leverage.toJSON(),
      mintAddress: this.mintAddress.toString(),
      periodData: this.periodData.toJSON(),
      providerPoolSize: this.providerPoolSize.toString(),
      userPoolSize: this.userPoolSize.toString(),
      utSupply: this.utSupply.toString(),
      ptSupply: this.ptSupply.toString(),
    }
  }

  static fromJSON(obj: SlaJSON): Sla {
    return new Sla({
      slaDeployerAddress: new PublicKey(obj.slaDeployerAddress),
      messengerAddress: new PublicKey(obj.messengerAddress),
      aggregatorAddress: new PublicKey(obj.aggregatorAddress),
      slo: types.Slo.fromJSON(obj.slo),
      leverage: types.DslaDecimal.fromJSON(obj.leverage),
      mintAddress: new PublicKey(obj.mintAddress),
      periodData: types.PeriodGenerator.fromJSON(obj.periodData),
      providerPoolSize: new BN(obj.providerPoolSize),
      userPoolSize: new BN(obj.userPoolSize),
      utSupply: new BN(obj.utSupply),
      ptSupply: new BN(obj.ptSupply),
    })
  }
}
