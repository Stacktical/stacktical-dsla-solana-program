import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh";

export interface InvalidSwitchboardAccountJSON {
  kind: "InvalidSwitchboardAccount";
}

export class InvalidSwitchboardAccount {
  static readonly discriminator = 0;
  static readonly kind = "InvalidSwitchboardAccount";
  readonly discriminator = 0;
  readonly kind = "InvalidSwitchboardAccount";

  toJSON(): InvalidSwitchboardAccountJSON {
    return {
      kind: "InvalidSwitchboardAccount",
    };
  }

  toEncodable() {
    return {
      InvalidSwitchboardAccount: {},
    };
  }
}

export interface StaleFeedJSON {
  kind: "StaleFeed";
}

export class StaleFeed {
  static readonly discriminator = 1;
  static readonly kind = "StaleFeed";
  readonly discriminator = 1;
  readonly kind = "StaleFeed";

  toJSON(): StaleFeedJSON {
    return {
      kind: "StaleFeed",
    };
  }

  toEncodable() {
    return {
      StaleFeed: {},
    };
  }
}

export interface ConfidenceIntervalExceededJSON {
  kind: "ConfidenceIntervalExceeded";
}

export class ConfidenceIntervalExceeded {
  static readonly discriminator = 2;
  static readonly kind = "ConfidenceIntervalExceeded";
  readonly discriminator = 2;
  readonly kind = "ConfidenceIntervalExceeded";

  toJSON(): ConfidenceIntervalExceededJSON {
    return {
      kind: "ConfidenceIntervalExceeded",
    };
  }

  toEncodable() {
    return {
      ConfidenceIntervalExceeded: {},
    };
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.FeedErrorCodeKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object");
  }

  if ("InvalidSwitchboardAccount" in obj) {
    return new InvalidSwitchboardAccount();
  }
  if ("StaleFeed" in obj) {
    return new StaleFeed();
  }
  if ("ConfidenceIntervalExceeded" in obj) {
    return new ConfidenceIntervalExceeded();
  }

  throw new Error("Invalid enum object");
}

export function fromJSON(
  obj: types.FeedErrorCodeJSON
): types.FeedErrorCodeKind {
  switch (obj.kind) {
    case "InvalidSwitchboardAccount": {
      return new InvalidSwitchboardAccount();
    }
    case "StaleFeed": {
      return new StaleFeed();
    }
    case "ConfidenceIntervalExceeded": {
      return new ConfidenceIntervalExceeded();
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "InvalidSwitchboardAccount"),
    borsh.struct([], "StaleFeed"),
    borsh.struct([], "ConfidenceIntervalExceeded"),
  ]);
  if (property !== undefined) {
    return ret.replicate(property);
  }
  return ret;
}
