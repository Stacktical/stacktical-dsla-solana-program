import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh";

export interface SloFields {
  sloValue: types.DslaDecimalFields;
  sloType: types.SloTypeKind;
}

export interface SloJSON {
  sloValue: types.DslaDecimalJSON;
  sloType: types.SloTypeJSON;
}

/** `Slo` is service level obejective and contains a Decimal number that is the expected value and  SloType */
export class Slo {
  readonly sloValue: types.DslaDecimal;
  readonly sloType: types.SloTypeKind;

  constructor(fields: SloFields) {
    this.sloValue = new types.DslaDecimal({ ...fields.sloValue });
    this.sloType = fields.sloType;
  }

  static layout(property?: string) {
    return borsh.struct(
      [types.DslaDecimal.layout("sloValue"), types.SloType.layout("sloType")],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new Slo({
      sloValue: types.DslaDecimal.fromDecoded(obj.sloValue),
      sloType: types.SloType.fromDecoded(obj.sloType),
    });
  }

  static toEncodable(fields: SloFields) {
    return {
      sloValue: types.DslaDecimal.toEncodable(fields.sloValue),
      sloType: fields.sloType.toEncodable(),
    };
  }

  toJSON(): SloJSON {
    return {
      sloValue: this.sloValue.toJSON(),
      sloType: this.sloType.toJSON(),
    };
  }

  static fromJSON(obj: SloJSON): Slo {
    return new Slo({
      sloValue: types.DslaDecimal.fromJSON(obj.sloValue),
      sloType: types.SloType.fromJSON(obj.sloType),
    });
  }

  toEncodable() {
    return Slo.toEncodable(this);
  }
}
