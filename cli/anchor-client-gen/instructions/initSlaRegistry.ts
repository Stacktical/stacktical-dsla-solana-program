import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types"; // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId";

export interface InitSlaRegistryAccounts {
  deployer: PublicKey;
  slaRegistry: PublicKey;
  systemProgram: PublicKey;
}

export function initSlaRegistry(accounts: InitSlaRegistryAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.deployer, isSigner: true, isWritable: true },
    { pubkey: accounts.slaRegistry, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ];
  const identifier = Buffer.from([20, 58, 193, 30, 243, 195, 230, 15]);
  const data = identifier;
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data });
  return ix;
}
