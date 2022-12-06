import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface StatusRegistryFields {
  statusRegistry: Array<types.StatusKind>
  bump: number
}

export interface StatusRegistryJSON {
  statusRegistry: Array<types.StatusJSON>
  bump: number
}

/** the registry with the stored status of each period after validation */
export class StatusRegistry {
  readonly statusRegistry: Array<types.StatusKind>
  readonly bump: number

  static readonly discriminator = Buffer.from([
    2, 194, 176, 5, 232, 56, 183, 193,
  ])

  static readonly layout = borsh.struct([
    borsh.vec(types.Status.layout(), "statusRegistry"),
    borsh.u8("bump"),
  ])

  constructor(fields: StatusRegistryFields) {
    this.statusRegistry = fields.statusRegistry
    this.bump = fields.bump
  }

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<StatusRegistry | null> {
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
  ): Promise<Array<StatusRegistry | null>> {
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

  static decode(data: Buffer): StatusRegistry {
    if (!data.slice(0, 8).equals(StatusRegistry.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = StatusRegistry.layout.decode(data.slice(8))

    return new StatusRegistry({
      statusRegistry: dec.statusRegistry.map(
        (
          item: any /* eslint-disable-line @typescript-eslint/no-explicit-any */
        ) => types.Status.fromDecoded(item)
      ),
      bump: dec.bump,
    })
  }

  toJSON(): StatusRegistryJSON {
    return {
      statusRegistry: this.statusRegistry.map((item) => item.toJSON()),
      bump: this.bump,
    }
  }

  static fromJSON(obj: StatusRegistryJSON): StatusRegistry {
    return new StatusRegistry({
      statusRegistry: obj.statusRegistry.map((item) =>
        types.Status.fromJSON(item)
      ),
      bump: obj.bump,
    })
  }
}
