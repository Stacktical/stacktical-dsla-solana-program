import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SlaRegistryFields {
  slaAccountAddresses: Array<PublicKey>
}

export interface SlaRegistryJSON {
  slaAccountAddresses: Array<string>
}

/** the `SlaRegistry` is an account with a vector containing the public keys for all SLAs */
export class SlaRegistry {
  readonly slaAccountAddresses: Array<PublicKey>

  static readonly discriminator = Buffer.from([
    95, 29, 91, 241, 143, 43, 156, 245,
  ])

  static readonly layout = borsh.struct([
    borsh.vec(borsh.publicKey(), "slaAccountAddresses"),
  ])

  constructor(fields: SlaRegistryFields) {
    this.slaAccountAddresses = fields.slaAccountAddresses
  }

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<SlaRegistry | null> {
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
  ): Promise<Array<SlaRegistry | null>> {
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

  static decode(data: Buffer): SlaRegistry {
    if (!data.slice(0, 8).equals(SlaRegistry.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = SlaRegistry.layout.decode(data.slice(8))

    return new SlaRegistry({
      slaAccountAddresses: dec.slaAccountAddresses,
    })
  }

  toJSON(): SlaRegistryJSON {
    return {
      slaAccountAddresses: this.slaAccountAddresses.map((item) =>
        item.toString()
      ),
    }
  }

  static fromJSON(obj: SlaRegistryJSON): SlaRegistry {
    return new SlaRegistry({
      slaAccountAddresses: obj.slaAccountAddresses.map(
        (item) => new PublicKey(item)
      ),
    })
  }
}
