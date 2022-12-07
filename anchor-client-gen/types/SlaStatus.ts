import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh";

export interface NotStartedJSON {
  kind: "NotStarted";
}

export class NotStarted {
  static readonly discriminator = 0;
  static readonly kind = "NotStarted";
  readonly discriminator = 0;
  readonly kind = "NotStarted";

  toJSON(): NotStartedJSON {
    return {
      kind: "NotStarted",
    };
  }

  toEncodable() {
    return {
      NotStarted: {},
    };
  }
}

export type ActiveFields = {
  periodId: number;
};
export type ActiveValue = {
  periodId: number;
};

export interface ActiveJSON {
  kind: "Active";
  value: {
    periodId: number;
  };
}

export class Active {
  static readonly discriminator = 1;
  static readonly kind = "Active";
  readonly discriminator = 1;
  readonly kind = "Active";
  readonly value: ActiveValue;

  constructor(value: ActiveFields) {
    this.value = {
      periodId: value.periodId,
    };
  }

  toJSON(): ActiveJSON {
    return {
      kind: "Active",
      value: {
        periodId: this.value.periodId,
      },
    };
  }

  toEncodable() {
    return {
      Active: {
        period_id: this.value.periodId,
      },
    };
  }
}

export interface EndedJSON {
  kind: "Ended";
}

export class Ended {
  static readonly discriminator = 2;
  static readonly kind = "Ended";
  readonly discriminator = 2;
  readonly kind = "Ended";

  toJSON(): EndedJSON {
    return {
      kind: "Ended",
    };
  }

  toEncodable() {
    return {
      Ended: {},
    };
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.SlaStatusKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object");
  }

  if ("NotStarted" in obj) {
    return new NotStarted();
  }
  if ("Active" in obj) {
    const val = obj["Active"];
    return new Active({
      periodId: val["period_id"],
    });
  }
  if ("Ended" in obj) {
    return new Ended();
  }

  throw new Error("Invalid enum object");
}

export function fromJSON(obj: types.SlaStatusJSON): types.SlaStatusKind {
  switch (obj.kind) {
    case "NotStarted": {
      return new NotStarted();
    }
    case "Active": {
      return new Active({
        periodId: obj.value.periodId,
      });
    }
    case "Ended": {
      return new Ended();
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "NotStarted"),
    borsh.struct([borsh.u32("period_id")], "Active"),
    borsh.struct([], "Ended"),
  ]);
  if (property !== undefined) {
    return ret.replicate(property);
  }
  return ret;
}
