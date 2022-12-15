import { Keypair, PublicKey } from "@solana/web3.js";
export const SLA_REGISTRY_SPACE = 5000; // MAX 10 MB
import BN from "bn.js";
import {
  PeriodLength,
  Slo,
  SloType,
  DslaDecimal,
} from "../anchor-client-gen/types";
// SEEDS
export const SLA_AUTHORITY_SEED = "sla-authority";
export const STATUS_REGISTRY_SEED = "status-registry";
export const DSLA_POOL_SEED = "dsla-vault";
export const POOL_SEED = "vault";
export const UT_MINT_SEED = "ut-mint";
export const PT_MINT_SEED = "pt-mint";
export const UT_ACCOUNT_SEED = "ut-account";
export const PT_ACCOUNT_SEED = "pt-account";
export const GOVERNANCE_SEED = "governance";
export const REWARD_SEED = "reward";
export const PERIOD_GENERATOR_SEED = "period-generator";
export const LOCKUP_PROVIDER_SEED = "provider-lockup";
export const LOCKUP_USER_SEED = "user-lockup";

// KEYPAIRS
export const SLA_PROTOCOL_DEPLOYER = new Keypair({
  publicKey: Uint8Array.from([
    131, 218, 37, 152, 136, 89, 103, 247, 100, 150, 178, 36, 20, 126, 167, 1,
    62, 222, 89, 85, 140, 166, 178, 104, 13, 240, 220, 225, 14, 40, 31, 138,
  ]),
  secretKey: Uint8Array.from([
    56, 189, 248, 64, 27, 134, 132, 60, 134, 157, 195, 19, 58, 95, 51, 132, 55,
    97, 137, 140, 73, 81, 38, 120, 237, 204, 113, 205, 132, 69, 21, 18, 131,
    218, 37, 152, 136, 89, 103, 247, 100, 150, 178, 36, 20, 126, 167, 1, 62,
    222, 89, 85, 140, 166, 178, 104, 13, 240, 220, 225, 14, 40, 31, 138,
  ]),
});

// DEVNET ADDRESSESS
export const SLA_REGISTRY_ADDRESS = new Keypair({
  publicKey: new PublicKey(
    "EhvvxtcmjriJCFGTCBR8Ng81KhsbmSVw1rEXn4ja7nvg"
  ).toBuffer(),
  secretKey: Uint8Array.from([
    5, 222, 13, 237, 210, 14, 68, 39, 28, 107, 232, 16, 48, 86, 197, 43, 249,
    19, 198, 164, 255, 57, 113, 233, 188, 64, 2, 200, 221, 229, 252, 109, 203,
    164, 36, 71, 51, 155, 230, 95, 87, 82, 195, 221, 154, 88, 185, 93, 188, 90,
    157, 1, 28, 83, 178, 254, 86, 189, 210, 24, 165, 144, 82, 253,
  ]),
});
export const DSLA_MINT = new PublicKey(
  "F9Q9oG47N9P3GbwiD7p5VKYZ1Sw2VawnoA6KyxHxUTRj"
);
export const RANDOM_MINT = new PublicKey(
  "CiQQJhe9gp7Z2ruNY2Ck7UGQktMwNFkjWppbbPeWGpz4"
);
export const AGGREGATOR_ADDRESS = new PublicKey(
  "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR"
);

export const SLA_ADDRESS = new PublicKey(
  "9hGTvj7WbCrduFwh8UFNjT6Qzh4xMXcNHU1whQ1cwgbv"
);

// VARIABLES
// Configure the client to use the env cluster.
const dslaDepositByPeriod = 250000000;
export const GOVERNANCE_PARAMETERS = {
  dslaDepositByPeriod: new BN(dslaDepositByPeriod),
  dslaProtocolReward: new BN(dslaDepositByPeriod * 0.5), // 50%
  dslaValidatorReward: new BN(dslaDepositByPeriod * 0.25), // 25%
  dslaBurnedByVerification: new BN(dslaDepositByPeriod * 0.25), // 25%,
  slaDeployerRewardsRate: new DslaDecimal({
    mantissa: new BN("3"),
    scale: 3,
  }), // 0.3%
  protocolRewardsRate: new DslaDecimal({
    mantissa: new BN("15"),
    scale: 4,
  }), // 0.15%
  maxLeverage: new DslaDecimal({ mantissa: new BN(1), scale: 0 }),
};

export const SLOS = [
  new Slo({
    sloValue: new DslaDecimal({
      mantissa: new BN("1396"),
      scale: 2,
    }),
    sloType: new SloType.GreaterThan(),
  }),
  new Slo({
    sloValue: new DslaDecimal({
      mantissa: new BN("12407625"),
      scale: 4,
    }),
    sloType: new SloType.EqualTo(),
  }),
  new Slo({
    sloValue: new DslaDecimal({
      mantissa: new BN("100"),
      scale: 0,
    }),
    sloType: new SloType.SmallerThan(),
  }),
];

export const SLAS = [
  {
    slo: SLOS[0],
    leverage: new DslaDecimal({ mantissa: new BN("1"), scale: 0 }),
    start: new BN(1671105900), // starts in 30 secs
    nPeriods: 100,
    periodLength: new PeriodLength.Custom({
      length: new BN(60), //each period is 1 minute
    }),
  },
  {
    slo: SLOS[1],
    leverage: new DslaDecimal({ mantissa: new BN("2"), scale: 0 }),
    start: new BN(Date.now() / 1000 + 2000),
    nPeriods: 1,
    periodLength: new PeriodLength.Custom({
      length: new BN(1000 * 60),
    }),
  },
  {
    slo: SLOS[2],
    leverage: new DslaDecimal({ mantissa: new BN("05"), scale: 1 }),
    start: new BN(Date.now() / 1000 + 3000),
    nPeriods: 1000,
    periodLength: new PeriodLength.Custom({
      length: new BN(1000 * 60 * 60 * 24),
    }),
  },
];
