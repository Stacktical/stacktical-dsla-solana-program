import { PublicKey, Connection } from "@solana/web3.js";
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId";

export interface LockupFields {
  availableTokens: BN;
  lockedTokensPrev: BN;
  lockedTokens: BN;
  lockedFromPeriodId: BN;
}

export interface LockupJSON {
  availableTokens: string;
  lockedTokensPrev: string;
  lockedTokens: string;
  lockedFromPeriodId: string;
}

/**
 * account to keep track of tokens that need to be locked
 * rule should be if tokens was staked in the last period should stay staked for at least one full period
 */
export class Lockup {
  readonly availableTokens: BN;
  readonly lockedTokensPrev: BN;
  readonly lockedTokens: BN;
  readonly lockedFromPeriodId: BN;

  static readonly discriminator = Buffer.from([1, 45, 32, 32, 57, 81, 88, 67]);

  static readonly layout = borsh.struct([
    borsh.u64("availableTokens"),
    borsh.u64("lockedTokensPrev"),
    borsh.u64("lockedTokens"),
    borsh.u64("lockedFromPeriodId"),
  ]);

  constructor(fields: LockupFields) {
    this.availableTokens = fields.availableTokens;
    this.lockedTokensPrev = fields.lockedTokensPrev;
    this.lockedTokens = fields.lockedTokens;
    this.lockedFromPeriodId = fields.lockedFromPeriodId;
  }

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<Lockup | null> {
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
  ): Promise<Array<Lockup | null>> {
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

  static decode(data: Buffer): Lockup {
    if (!data.slice(0, 8).equals(Lockup.discriminator)) {
      throw new Error("invalid account discriminator");
    }

    const dec = Lockup.layout.decode(data.slice(8));

    return new Lockup({
      availableTokens: dec.availableTokens,
      lockedTokensPrev: dec.lockedTokensPrev,
      lockedTokens: dec.lockedTokens,
      lockedFromPeriodId: dec.lockedFromPeriodId,
    });
  }

  toJSON(): LockupJSON {
    return {
      availableTokens: this.availableTokens.toString(),
      lockedTokensPrev: this.lockedTokensPrev.toString(),
      lockedTokens: this.lockedTokens.toString(),
      lockedFromPeriodId: this.lockedFromPeriodId.toString(),
    };
  }

  static fromJSON(obj: LockupJSON): Lockup {
    return new Lockup({
      availableTokens: new BN(obj.availableTokens),
      lockedTokensPrev: new BN(obj.lockedTokensPrev),
      lockedTokens: new BN(obj.lockedTokens),
      lockedFromPeriodId: new BN(obj.lockedFromPeriodId),
    });
  }
}
