import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"

export type CustomFields = {
  length: BN
}
export type CustomValue = {
  length: BN
}

export interface CustomJSON {
  kind: "Custom"
  value: {
    length: string
  }
}

export class Custom {
  static readonly discriminator = 0
  static readonly kind = "Custom"
  readonly discriminator = 0
  readonly kind = "Custom"
  readonly value: CustomValue

  constructor(value: CustomFields) {
    this.value = {
      length: value.length,
    }
  }

  toJSON(): CustomJSON {
    return {
      kind: "Custom",
      value: {
        length: this.value.length.toString(),
      },
    }
  }

  toEncodable() {
    return {
      Custom: {
        length: this.value.length,
      },
    }
  }
}

export interface MonthlyJSON {
  kind: "Monthly"
}

export class Monthly {
  static readonly discriminator = 1
  static readonly kind = "Monthly"
  readonly discriminator = 1
  readonly kind = "Monthly"

  toJSON(): MonthlyJSON {
    return {
      kind: "Monthly",
    }
  }

  toEncodable() {
    return {
      Monthly: {},
    }
  }
}

export interface YearlyJSON {
  kind: "Yearly"
}

export class Yearly {
  static readonly discriminator = 2
  static readonly kind = "Yearly"
  readonly discriminator = 2
  readonly kind = "Yearly"

  toJSON(): YearlyJSON {
    return {
      kind: "Yearly",
    }
  }

  toEncodable() {
    return {
      Yearly: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.PeriodLengthKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("Custom" in obj) {
    const val = obj["Custom"]
    return new Custom({
      length: val["length"],
    })
  }
  if ("Monthly" in obj) {
    return new Monthly()
  }
  if ("Yearly" in obj) {
    return new Yearly()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(obj: types.PeriodLengthJSON): types.PeriodLengthKind {
  switch (obj.kind) {
    case "Custom": {
      return new Custom({
        length: new BN(obj.value.length),
      })
    }
    case "Monthly": {
      return new Monthly()
    }
    case "Yearly": {
      return new Yearly()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([borsh.u128("length")], "Custom"),
    borsh.struct([], "Monthly"),
    borsh.struct([], "Yearly"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
