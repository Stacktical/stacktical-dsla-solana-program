import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh";

export interface NotVerifiedJSON {
  kind: "NotVerified";
}

export class NotVerified {
  static readonly discriminator = 0;
  static readonly kind = "NotVerified";
  readonly discriminator = 0;
  readonly kind = "NotVerified";

  toJSON(): NotVerifiedJSON {
    return {
      kind: "NotVerified",
    };
  }

  toEncodable() {
    return {
      NotVerified: {},
    };
  }
}

export type RespectedFields = {
  value: types.DslaDecimalFields;
};
export type RespectedValue = {
  value: types.DslaDecimal;
};

export interface RespectedJSON {
  kind: "Respected";
  value: {
    value: types.DslaDecimalJSON;
  };
}

export class Respected {
  static readonly discriminator = 1;
  static readonly kind = "Respected";
  readonly discriminator = 1;
  readonly kind = "Respected";
  readonly value: RespectedValue;

  constructor(value: RespectedFields) {
    this.value = {
      value: new types.DslaDecimal({ ...value.value }),
    };
  }

  toJSON(): RespectedJSON {
    return {
      kind: "Respected",
      value: {
        value: this.value.value.toJSON(),
      },
    };
  }

  toEncodable() {
    return {
      Respected: {
        value: types.DslaDecimal.toEncodable(this.value.value),
      },
    };
  }
}

export type NotRespectedFields = {
  value: types.DslaDecimalFields;
};
export type NotRespectedValue = {
  value: types.DslaDecimal;
};

export interface NotRespectedJSON {
  kind: "NotRespected";
  value: {
    value: types.DslaDecimalJSON;
  };
}

export class NotRespected {
  static readonly discriminator = 2;
  static readonly kind = "NotRespected";
  readonly discriminator = 2;
  readonly kind = "NotRespected";
  readonly value: NotRespectedValue;

  constructor(value: NotRespectedFields) {
    this.value = {
      value: new types.DslaDecimal({ ...value.value }),
    };
  }

  toJSON(): NotRespectedJSON {
    return {
      kind: "NotRespected",
      value: {
        value: this.value.value.toJSON(),
      },
    };
  }

  toEncodable() {
    return {
      NotRespected: {
        value: types.DslaDecimal.toEncodable(this.value.value),
      },
    };
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.StatusKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object");
  }

  if ("NotVerified" in obj) {
    return new NotVerified();
  }
  if ("Respected" in obj) {
    const val = obj["Respected"];
    return new Respected({
      value: types.DslaDecimal.fromDecoded(val["value"]),
    });
  }
  if ("NotRespected" in obj) {
    const val = obj["NotRespected"];
    return new NotRespected({
      value: types.DslaDecimal.fromDecoded(val["value"]),
    });
  }

  throw new Error("Invalid enum object");
}

export function fromJSON(obj: types.StatusJSON): types.StatusKind {
  switch (obj.kind) {
    case "NotVerified": {
      return new NotVerified();
    }
    case "Respected": {
      return new Respected({
        value: types.DslaDecimal.fromJSON(obj.value.value),
      });
    }
    case "NotRespected": {
      return new NotRespected({
        value: types.DslaDecimal.fromJSON(obj.value.value),
      });
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "NotVerified"),
    borsh.struct([types.DslaDecimal.layout("value")], "Respected"),
    borsh.struct([types.DslaDecimal.layout("value")], "NotRespected"),
  ]);
  if (property !== undefined) {
    return ret.replicate(property);
  }
  return ret;
}
