import { PublicKey, Connection } from "@solana/web3.js";
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId";

export interface SlaAuthorityFields {}

export interface SlaAuthorityJSON {}

export class SlaAuthority {
  static readonly discriminator = Buffer.from([
    188, 127, 254, 192, 35, 167, 17, 71,
  ]);

  static readonly layout = borsh.struct([]);

  constructor(fields: SlaAuthorityFields) {}

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<SlaAuthority | null> {
    const info = await c.getAccountInfo(address);

    if (info === null) {
      return null;
    }
    if (!info.owner.equals(PROGRAM_ID)) {
      throw new Error("account doesn't belong to this program");
    }

    return this.decode(info.data);
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[]
  ): Promise<Array<SlaAuthority | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses);

    return infos.map((info) => {
      if (info === null) {
        return null;
      }
      if (!info.owner.equals(PROGRAM_ID)) {
        throw new Error("account doesn't belong to this program");
      }

      return this.decode(info.data);
    });
  }

  static decode(data: Buffer): SlaAuthority {
    if (!data.slice(0, 8).equals(SlaAuthority.discriminator)) {
      throw new Error("invalid account discriminator");
    }

    const dec = SlaAuthority.layout.decode(data.slice(8));

    return new SlaAuthority({});
  }

  toJSON(): SlaAuthorityJSON {
    return {};
  }

  static fromJSON(obj: SlaAuthorityJSON): SlaAuthority {
    return new SlaAuthority({});
  }
}
