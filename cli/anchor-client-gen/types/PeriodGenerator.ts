import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh";

export interface PeriodGeneratorFields {
  /** the first timestamp indicating the beginning of the SLA and of the first period */
  start: BN;
  /** the length of each period */
  periodLength: types.PeriodLengthKind;
  /** number of periods */
  nPeriods: number;
}

export interface PeriodGeneratorJSON {
  /** the first timestamp indicating the beginning of the SLA and of the first period */
  start: string;
  /** the length of each period */
  periodLength: types.PeriodLengthJSON;
  /** number of periods */
  nPeriods: number;
}

/** struct used to generate the periods for an SLA with helper function to retrieve any period */
export class PeriodGenerator {
  /** the first timestamp indicating the beginning of the SLA and of the first period */
  readonly start: BN;
  /** the length of each period */
  readonly periodLength: types.PeriodLengthKind;
  /** number of periods */
  readonly nPeriods: number;

  constructor(fields: PeriodGeneratorFields) {
    this.start = fields.start;
    this.periodLength = fields.periodLength;
    this.nPeriods = fields.nPeriods;
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u128("start"),
        types.PeriodLength.layout("periodLength"),
        borsh.u32("nPeriods"),
      ],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new PeriodGenerator({
      start: obj.start,
      periodLength: types.PeriodLength.fromDecoded(obj.periodLength),
      nPeriods: obj.nPeriods,
    });
  }

  static toEncodable(fields: PeriodGeneratorFields) {
    return {
      start: fields.start,
      periodLength: fields.periodLength.toEncodable(),
      nPeriods: fields.nPeriods,
    };
  }

  toJSON(): PeriodGeneratorJSON {
    return {
      start: this.start.toString(),
      periodLength: this.periodLength.toJSON(),
      nPeriods: this.nPeriods,
    };
  }

  static fromJSON(obj: PeriodGeneratorJSON): PeriodGenerator {
    return new PeriodGenerator({
      start: new BN(obj.start),
      periodLength: types.PeriodLength.fromJSON(obj.periodLength),
      nPeriods: obj.nPeriods,
    });
  }

  toEncodable() {
    return PeriodGenerator.toEncodable(this);
  }
}
