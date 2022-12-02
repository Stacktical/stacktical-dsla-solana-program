import { Keypair, PublicKey } from "@solana/web3.js";
export const SLA_REGISTRY_SPACE = 10_000_000;
import { BN } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";

export const PROVIDER = anchor.AnchorProvider.env();

// SEEDS
export const STATUS_REGISTRY_SEED: string = "status-registry";
export const POOL_SEED: string = "vault";
export const UT_MINT_SEED: string = "ut-mint";
export const PT_MINT_SEED: string = "pt-mint";
export const GOVERNANCE_SEED: string = "governance";
export const UT_ACCOUNT_SEED: string = "ut-account";
export const PT_ACCOUNT_SEED: string = "pt-account";

// KEYPAIRS
export const SLA_REGISTRY_DEPLOYER = Keypair.generate();
export const SLA_REGISTRY_KEYPAIR = Keypair.generate();
export const MINT_AUTHORITY: Keypair = Keypair.generate();
export const DSLA_MINT_AUTHORITY: Keypair = Keypair.generate();
export const SLA_PROTOCOL_DEPLOYER = {
  publicKey: new PublicKey([
    131, 218, 37, 152, 136, 89, 103, 247, 100, 150, 178, 36, 20, 126, 167, 1,
    62, 222, 89, 85, 140, 166, 178, 104, 13, 240, 220, 225, 14, 40, 31, 138,
  ]),
  secretKey: Uint8Array.from([
    56, 189, 248, 64, 27, 134, 132, 60, 134, 157, 195, 19, 58, 95, 51, 132, 55,
    97, 137, 140, 73, 81, 38, 120, 237, 204, 113, 205, 132, 69, 21, 18, 131,
    218, 37, 152, 136, 89, 103, 247, 100, 150, 178, 36, 20, 126, 167, 1, 62,
    222, 89, 85, 140, 166, 178, 104, 13, 240, 220, 225, 14, 40, 31, 138,
  ]),
};

export const SLA_DEPLOYERS: Keypair[] = [
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate(),
];
export const SLA_KEYPAIRS: Keypair[] = [
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate(),
];
export const STAKERS: Keypair[] = [
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate(),
];
export const MESSENGER_ADDRESSES: PublicKey[] = [
  Keypair.generate().publicKey,
  Keypair.generate().publicKey,
  Keypair.generate().publicKey,
];

// VARIABLES
// Configure the client to use the env cluster.
let dslaDepositByPeriod = 250000000;
export const GOVERNANCE_PARAMETERS = {
  dslaDepositByPeriod: new BN(dslaDepositByPeriod),
  dslaProtocolReward: new BN(dslaDepositByPeriod * 0.5), // 50%
  dslaValidatorReward: new BN(dslaDepositByPeriod * 0.25), // 25%
  dslaBurnedByVerification: new BN(dslaDepositByPeriod * 0.25), // 25%,
  slaDeployerRewardsRate: {
    mantissa: new BN("3"),
    scale: new BN("3"),
  }, // 0.3%
  protocolRewardsRate: {
    mantissa: new BN("15"),
    scale: new BN("4"),
  }, // 0.15%
  maxLeverage: new BN(10),
  burnDsla: true,
};
export const SLOS = [
  {
    sloValue: {
      mantissa: new BN("100"),
      scale: new BN("0"),
    },
    sloType: { greaterThan: {} },
  },
  {
    sloValue: {
      mantissa: new BN("1923"),
      scale: new BN("4"),
    },
    sloType: { equalTo: {} },
  },
  {
    sloValue: {
      mantissa: new BN("100"),
      scale: new BN("0"),
    },
    sloType: { smallerThan: {} },
  },
];
export const SLAS = [
  {
    id: 0,
    slo: SLOS[0],
    messengerAddress: MESSENGER_ADDRESSES[0],
    leverage: { mantissa: new BN("1"), scale: new BN("0") },
    start: new BN(Date.now() + 1000),
    nPeriods: new BN("100"),
    periodLength: { custom: { length: new BN(1000 * 60 * 60) } },
  },
  {
    id: 1,
    slo: SLOS[1],
    messengerAddress: MESSENGER_ADDRESSES[1],
    leverage: { mantissa: new BN("2"), scale: new BN("0") },
    start: new BN(Date.now() + 2000),
    nPeriods: new BN("1"),
    periodLength: { custom: { length: new BN(1000 * 60) } },
  },
  {
    id: 2,
    slo: SLOS[2],
    messengerAddress: MESSENGER_ADDRESSES[2],
    leverage: { mantissa: new BN("05"), scale: new BN("1") },
    start: new BN(Date.now() + 3000),
    nPeriods: new BN("1000"),
    periodLength: { custom: { length: new BN(1000 * 60 * 60 * 24) } },
  },
];
