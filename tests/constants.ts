import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
export const STATUS_REGISTRY_SEED: string = "status-registry";
export const USER_POOL_SEED: string = "user-vault";
export const PROVIDER_POOL_SEED: string = "provider-vault";
export const UT_MINT_SEED: string = "ut-mint";
export const PT_MINT_SEED: string = "pt-mint";
export const GOVERNANCE_SEED: string = "governance";
export const UT_ACCOUNT_SEED: string = "ut-account";
export const PT_ACCOUNT_SEED: string = "pt-account";
export const SLA_REGISTRY_SPACE = 10_000_000;

export const DEPLOYER = {
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
async function generateSlaList() {}
export const GOVERNANCE_PARAMETERS = {
  dslaBurnRate: new anchor.BN(10),
  dslaDepositByPeriod: new anchor.BN(10),
  dslaPlatformReward: new anchor.BN(10),
  dslaMessengerReward: new anchor.BN(10),
  dslaUserReward: new anchor.BN(10),
  dslaBurnedByVerification: new anchor.BN(10),
  maxTokenLength: new anchor.BN(10),
  maxLeverage: new anchor.BN(10),
  burnDsla: true,
};
