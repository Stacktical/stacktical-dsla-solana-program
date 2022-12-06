import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface GovernanceFields {
  /** amount of dsla to be deposited by the sla_deployer to deploy the sla for each period */
  dslaDepositByPeriod: BN
  /** amount of dsla deposit by period to be given to the platform */
  dslaProtocolReward: BN
  /** amount of dsla deposit by period to be given to the validator */
  dslaValidatorReward: BN
  /** amount of dsla deposit by period to be burned */
  dslaBurnedByVerification: BN
  /** percentage of withdrawal to be payed to the Deployer of the SLA */
  slaDeployerRewardsRate: types.DslaDecimalFields
  /** percentage of withdrawal to be payed to the Deployer of the DSLA protocol */
  protocolRewardsRate: types.DslaDecimalFields
  /** max leverage allowed in a DSLA */
  maxLeverage: types.DslaDecimalFields
}

export interface GovernanceJSON {
  /** amount of dsla to be deposited by the sla_deployer to deploy the sla for each period */
  dslaDepositByPeriod: string
  /** amount of dsla deposit by period to be given to the platform */
  dslaProtocolReward: string
  /** amount of dsla deposit by period to be given to the validator */
  dslaValidatorReward: string
  /** amount of dsla deposit by period to be burned */
  dslaBurnedByVerification: string
  /** percentage of withdrawal to be payed to the Deployer of the SLA */
  slaDeployerRewardsRate: types.DslaDecimalJSON
  /** percentage of withdrawal to be payed to the Deployer of the DSLA protocol */
  protocolRewardsRate: types.DslaDecimalJSON
  /** max leverage allowed in a DSLA */
  maxLeverage: types.DslaDecimalJSON
}

/** collection fo all the parametric Governances one account for all SLAs */
export class Governance {
  /** amount of dsla to be deposited by the sla_deployer to deploy the sla for each period */
  readonly dslaDepositByPeriod: BN
  /** amount of dsla deposit by period to be given to the platform */
  readonly dslaProtocolReward: BN
  /** amount of dsla deposit by period to be given to the validator */
  readonly dslaValidatorReward: BN
  /** amount of dsla deposit by period to be burned */
  readonly dslaBurnedByVerification: BN
  /** percentage of withdrawal to be payed to the Deployer of the SLA */
  readonly slaDeployerRewardsRate: types.DslaDecimal
  /** percentage of withdrawal to be payed to the Deployer of the DSLA protocol */
  readonly protocolRewardsRate: types.DslaDecimal
  /** max leverage allowed in a DSLA */
  readonly maxLeverage: types.DslaDecimal

  static readonly discriminator = Buffer.from([
    18, 143, 88, 13, 73, 217, 47, 49,
  ])

  static readonly layout = borsh.struct([
    borsh.u64("dslaDepositByPeriod"),
    borsh.u64("dslaProtocolReward"),
    borsh.u64("dslaValidatorReward"),
    borsh.u64("dslaBurnedByVerification"),
    types.DslaDecimal.layout("slaDeployerRewardsRate"),
    types.DslaDecimal.layout("protocolRewardsRate"),
    types.DslaDecimal.layout("maxLeverage"),
  ])

  constructor(fields: GovernanceFields) {
    this.dslaDepositByPeriod = fields.dslaDepositByPeriod
    this.dslaProtocolReward = fields.dslaProtocolReward
    this.dslaValidatorReward = fields.dslaValidatorReward
    this.dslaBurnedByVerification = fields.dslaBurnedByVerification
    this.slaDeployerRewardsRate = new types.DslaDecimal({
      ...fields.slaDeployerRewardsRate,
    })
    this.protocolRewardsRate = new types.DslaDecimal({
      ...fields.protocolRewardsRate,
    })
    this.maxLeverage = new types.DslaDecimal({ ...fields.maxLeverage })
  }

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<Governance | null> {
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
  ): Promise<Array<Governance | null>> {
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

  static decode(data: Buffer): Governance {
    if (!data.slice(0, 8).equals(Governance.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = Governance.layout.decode(data.slice(8))

    return new Governance({
      dslaDepositByPeriod: dec.dslaDepositByPeriod,
      dslaProtocolReward: dec.dslaProtocolReward,
      dslaValidatorReward: dec.dslaValidatorReward,
      dslaBurnedByVerification: dec.dslaBurnedByVerification,
      slaDeployerRewardsRate: types.DslaDecimal.fromDecoded(
        dec.slaDeployerRewardsRate
      ),
      protocolRewardsRate: types.DslaDecimal.fromDecoded(
        dec.protocolRewardsRate
      ),
      maxLeverage: types.DslaDecimal.fromDecoded(dec.maxLeverage),
    })
  }

  toJSON(): GovernanceJSON {
    return {
      dslaDepositByPeriod: this.dslaDepositByPeriod.toString(),
      dslaProtocolReward: this.dslaProtocolReward.toString(),
      dslaValidatorReward: this.dslaValidatorReward.toString(),
      dslaBurnedByVerification: this.dslaBurnedByVerification.toString(),
      slaDeployerRewardsRate: this.slaDeployerRewardsRate.toJSON(),
      protocolRewardsRate: this.protocolRewardsRate.toJSON(),
      maxLeverage: this.maxLeverage.toJSON(),
    }
  }

  static fromJSON(obj: GovernanceJSON): Governance {
    return new Governance({
      dslaDepositByPeriod: new BN(obj.dslaDepositByPeriod),
      dslaProtocolReward: new BN(obj.dslaProtocolReward),
      dslaValidatorReward: new BN(obj.dslaValidatorReward),
      dslaBurnedByVerification: new BN(obj.dslaBurnedByVerification),
      slaDeployerRewardsRate: types.DslaDecimal.fromJSON(
        obj.slaDeployerRewardsRate
      ),
      protocolRewardsRate: types.DslaDecimal.fromJSON(obj.protocolRewardsRate),
      maxLeverage: types.DslaDecimal.fromJSON(obj.maxLeverage),
    })
  }
}
