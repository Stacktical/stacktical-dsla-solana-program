import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
export const SLA_REGISTRY_SPACE = 10_000_000;

export const STATUS_REGISTRY_SEED: string = "status-registry";
export const POOL_SEED: string = "vault";
export const UT_MINT_SEED: string = "ut-mint";
export const PT_MINT_SEED: string = "pt-mint";
export const GOVERNANCE_SEED: string = "governance";
export const UT_ACCOUNT_SEED: string = "ut-account";
export const PT_ACCOUNT_SEED: string = "pt-account";

export const SLA_REGISTRY_DEPLOYER = {
  publicKey: new PublicKey("7AnuoPY7GqB1MdwLnSoeQh5YULCD6hjyFfSv2yPqYUfu"),
  secretKey: Uint8Array.from([
    127, 39, 233, 13, 12, 209, 1, 226, 123, 216, 96, 124, 99, 211, 222, 148,
    210, 123, 173, 169, 130, 103, 89, 231, 254, 226, 170, 171, 76, 92, 51, 5,
    91, 168, 66, 132, 28, 26, 23, 136, 13, 53, 237, 144, 209, 110, 29, 246, 135,
    73, 120, 68, 148, 215, 68, 155, 164, 217, 114, 218, 165, 183, 212, 180,
  ]),
};

export const SLA_REGISTRY_KEYPAIR = {
  publicKey: new PublicKey("4uGypsS5XE2h7tLXa3HtHDQpMr5v6NgqMA6VZQQVVffv"),
  secretKey: Uint8Array.from([
    79, 172, 35, 99, 162, 199, 50, 164, 166, 116, 154, 92, 132, 254, 215, 214,
    55, 65, 192, 121, 189, 150, 78, 158, 71, 27, 193, 202, 67, 131, 78, 80, 57,
    247, 84, 202, 23, 239, 56, 67, 202, 63, 155, 249, 46, 29, 86, 125, 244, 201,
    18, 191, 102, 208, 239, 204, 54, 15, 163, 41, 14, 251, 217, 105,
  ]),
};

export const MINT_AUTHORITY: Keypair = Keypair.generate();
export const DSLA_MINT_AUTHORITY: Keypair = Keypair.generate();
export const GOVERNANCE_PARAMETERS = {
  dslaDepositByPeriod: new anchor.BN("250000000"),
  dslaProtocolRewardRate: {
    mantissa: new anchor.BN("50"),
    scale: new anchor.BN("2"),
  }, // 50%
  dslaValidatorRewardRate: {
    mantissa: new anchor.BN("25"),
    scale: new anchor.BN("2"),
  }, // 25%
  dslaBurnedByVerificationRate: {
    mantissa: new anchor.BN("25"),
    scale: new anchor.BN("2"),
  }, // 25%,
  slaDeployerRewardsRate: {
    mantissa: new anchor.BN("3"),
    scale: new anchor.BN("3"),
  }, // 0.3%
  protocolRewardsRate: {
    mantissa: new anchor.BN("15"),
    scale: new anchor.BN("4"),
  }, // 0.15%
  maxLeverage: new anchor.BN(10),
  burnDsla: true,
};

export const SLA_PROTOCOL_DEPLOYER: Keypair = Keypair.generate();

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

export const SLOS = [
  {
    sloValue: {
      mantissa: new anchor.BN("100"),
      scale: new anchor.BN("0"),
    },
    sloType: { greaterThan: {} },
  },
  {
    sloValue: {
      mantissa: new anchor.BN("1923"),
      scale: new anchor.BN("4"),
    },
    sloType: { equalTo: {} },
  },
  {
    sloValue: {
      mantissa: new anchor.BN("100"),
      scale: new anchor.BN("0"),
    },
    sloType: { smallerThan: {} },
  },
];

export const MESSENGER_ADDRESSES: PublicKey[] = [
  anchor.web3.Keypair.generate().publicKey,
  anchor.web3.Keypair.generate().publicKey,
  anchor.web3.Keypair.generate().publicKey,
];

export const SLAS = [
  {
    id: 0,
    slo: SLOS[0],
    messengerAddress: MESSENGER_ADDRESSES[0],
    leverage: { mantissa: new anchor.BN("1"), scale: new anchor.BN("0") },
    start: new anchor.BN(Date.now() + 1000),
    nPeriods: new anchor.BN("100"),
    periodLength: { custom: { length: new anchor.BN(1000 * 60 * 60) } },
  },
  {
    id: 1,
    slo: SLOS[1],
    messengerAddress: MESSENGER_ADDRESSES[1],
    leverage: { mantissa: new anchor.BN("2"), scale: new anchor.BN("0") },
    start: new anchor.BN(Date.now() + 2000),
    nPeriods: new anchor.BN("1"),
    periodLength: { custom: { length: new anchor.BN(1000 * 60) } },
  },
  {
    id: 2,
    slo: SLOS[2],
    messengerAddress: MESSENGER_ADDRESSES[2],
    leverage: { mantissa: new anchor.BN("05"), scale: new anchor.BN("1") },
    start: new anchor.BN(Date.now() + 3000),
    nPeriods: new anchor.BN("1000"),
    periodLength: { custom: { length: new anchor.BN(1000 * 60 * 60 * 24) } },
  },
];
