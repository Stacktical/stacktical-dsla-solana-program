import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh";

export interface EqualToJSON {
  kind: "EqualTo";
}

export class EqualTo {
  static readonly discriminator = 0;
  static readonly kind = "EqualTo";
  readonly discriminator = 0;
  readonly kind = "EqualTo";

  toJSON(): EqualToJSON {
    return {
      kind: "EqualTo",
    };
  }

  toEncodable() {
    return {
      EqualTo: {},
    };
  }
}

export interface NotEqualToJSON {
  kind: "NotEqualTo";
}

export class NotEqualTo {
  static readonly discriminator = 1;
  static readonly kind = "NotEqualTo";
  readonly discriminator = 1;
  readonly kind = "NotEqualTo";

  toJSON(): NotEqualToJSON {
    return {
      kind: "NotEqualTo",
    };
  }

  toEncodable() {
    return {
      NotEqualTo: {},
    };
  }
}

export interface SmallerThanJSON {
  kind: "SmallerThan";
}

export class SmallerThan {
  static readonly discriminator = 2;
  static readonly kind = "SmallerThan";
  readonly discriminator = 2;
  readonly kind = "SmallerThan";

  toJSON(): SmallerThanJSON {
    return {
      kind: "SmallerThan",
    };
  }

  toEncodable() {
    return {
      SmallerThan: {},
    };
  }
}

export interface SmallerOrEqualToJSON {
  kind: "SmallerOrEqualTo";
}

export class SmallerOrEqualTo {
  static readonly discriminator = 3;
  static readonly kind = "SmallerOrEqualTo";
  readonly discriminator = 3;
  readonly kind = "SmallerOrEqualTo";

  toJSON(): SmallerOrEqualToJSON {
    return {
      kind: "SmallerOrEqualTo",
    };
  }

  toEncodable() {
    return {
      SmallerOrEqualTo: {},
    };
  }
}

export interface GreaterThanJSON {
  kind: "GreaterThan";
}

export class GreaterThan {
  static readonly discriminator = 4;
  static readonly kind = "GreaterThan";
  readonly discriminator = 4;
  readonly kind = "GreaterThan";

  toJSON(): GreaterThanJSON {
    return {
      kind: "GreaterThan",
    };
  }

  toEncodable() {
    return {
      GreaterThan: {},
    };
  }
}

export interface GreaterOrEqualToJSON {
  kind: "GreaterOrEqualTo";
}

export class GreaterOrEqualTo {
  static readonly discriminator = 5;
  static readonly kind = "GreaterOrEqualTo";
  readonly discriminator = 5;
  readonly kind = "GreaterOrEqualTo";

  toJSON(): GreaterOrEqualToJSON {
    return {
      kind: "GreaterOrEqualTo",
    };
  }

  toEncodable() {
    return {
      GreaterOrEqualTo: {},
    };
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.SloTypeKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object");
  }

  if ("EqualTo" in obj) {
    return new EqualTo();
  }
  if ("NotEqualTo" in obj) {
    return new NotEqualTo();
  }
  if ("SmallerThan" in obj) {
    return new SmallerThan();
  }
  if ("SmallerOrEqualTo" in obj) {
    return new SmallerOrEqualTo();
  }
  if ("GreaterThan" in obj) {
    return new GreaterThan();
  }
  if ("GreaterOrEqualTo" in obj) {
    return new GreaterOrEqualTo();
  }

  throw new Error("Invalid enum object");
}

export function fromJSON(obj: types.SloTypeJSON): types.SloTypeKind {
  switch (obj.kind) {
    case "EqualTo": {
      return new EqualTo();
    }
    case "NotEqualTo": {
      return new NotEqualTo();
    }
    case "SmallerThan": {
      return new SmallerThan();
    }
    case "SmallerOrEqualTo": {
      return new SmallerOrEqualTo();
    }
    case "GreaterThan": {
      return new GreaterThan();
    }
    case "GreaterOrEqualTo": {
      return new GreaterOrEqualTo();
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "EqualTo"),
    borsh.struct([], "NotEqualTo"),
    borsh.struct([], "SmallerThan"),
    borsh.struct([], "SmallerOrEqualTo"),
    borsh.struct([], "GreaterThan"),
    borsh.struct([], "GreaterOrEqualTo"),
  ]);
  if (property !== undefined) {
    return ret.replicate(property);
  }
  return ret;
}
