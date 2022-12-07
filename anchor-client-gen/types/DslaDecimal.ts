import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh";

export interface DslaDecimalFields {
  mantissa: BN;
  scale: number;
}

export interface DslaDecimalJSON {
  mantissa: string;
  scale: number;
}

/**
 * struct to deal with floating point numbers
 * - `mantissa` the value without any decimals and non decimal
 * - `scale` how many places from the right to put the decimal point
 */
export class DslaDecimal {
  readonly mantissa: BN;
  readonly scale: number;

  constructor(fields: DslaDecimalFields) {
    this.mantissa = fields.mantissa;
    this.scale = fields.scale;
  }

  static layout(property?: string) {
    return borsh.struct([borsh.i64("mantissa"), borsh.u32("scale")], property);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new DslaDecimal({
      mantissa: obj.mantissa,
      scale: obj.scale,
    });
  }

  static toEncodable(fields: DslaDecimalFields) {
    return {
      mantissa: fields.mantissa,
      scale: fields.scale,
    };
  }

  toJSON(): DslaDecimalJSON {
    return {
      mantissa: this.mantissa.toString(),
      scale: this.scale,
    };
  }

  static fromJSON(obj: DslaDecimalJSON): DslaDecimal {
    return new DslaDecimal({
      mantissa: new BN(obj.mantissa),
      scale: obj.scale,
    });
  }

  toEncodable() {
    return DslaDecimal.toEncodable(this);
  }
}
