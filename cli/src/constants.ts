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
    "CKe5JsWvi4axgPwpiDVB5G3UkioVNNLoTvcCztNgMobM"
  ).toBuffer(),
  secretKey: Uint8Array.from([
    161, 132, 218, 215, 222, 94, 11, 251, 131, 71, 30, 31, 5, 119, 151, 181,
    220, 243, 66, 37, 227, 77, 78, 56, 114, 18, 154, 226, 231, 147, 225, 160,
    168, 54, 238, 71, 34, 119, 76, 147, 16, 215, 198, 203, 178, 58, 32, 163,
    161, 89, 214, 3, 34, 71, 251, 196, 115, 183, 240, 236, 233, 47, 101, 48,
  ]),
});
export const DSLA_MINT = new PublicKey(
  "F9Q9oG47N9P3GbwiD7p5VKYZ1Sw2VawnoA6KyxHxUTRj"
);
export const RANDOM_MINT = new PublicKey(
  "CiQQJhe9gp7Z2ruNY2Ck7UGQktMwNFkjWppbbPeWGpz4"
);
export const AGGREGATOR_ADDRESS = new PublicKey(
  "5fbq7xq86bWFxTySesUfxxU5HWiGgx6jh1girsqRPKei"
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
      mantissa: new BN("1240"),
      scale: 0,
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
    start: new BN(Date.now() + 1000),
    nPeriods: 100,
    periodLength: new PeriodLength.Custom({
      length: new BN(1000 * 60 * 60),
    }),
  },
  {
    slo: SLOS[1],
    leverage: new DslaDecimal({ mantissa: new BN("2"), scale: 0 }),
    start: new BN(Date.now() + 2000),
    nPeriods: 1,
    periodLength: new PeriodLength.Custom({
      length: new BN(1000 * 60),
    }),
  },
  {
    slo: SLOS[2],
    leverage: new DslaDecimal({ mantissa: new BN("05"), scale: 1 }),
    start: new BN(Date.now() + 3000),
    nPeriods: 1000,
    periodLength: new PeriodLength.Custom({
      length: new BN(1000 * 60 * 60 * 24),
    }),
  },
];
