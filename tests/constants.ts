import { Keypair, PublicKey } from "@solana/web3.js";
export const SLA_REGISTRY_SPACE = 10_000_000;
import { BN } from "@project-serum/anchor";

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
export const MESSENGER_ADDRESSES: PublicKey[] = [
  Keypair.generate().publicKey,
  Keypair.generate().publicKey,
  Keypair.generate().publicKey,
];

// VARIABLES
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
