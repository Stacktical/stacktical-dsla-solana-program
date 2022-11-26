import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

export async function fund_account(connection: Connection, pubkey: PublicKey) {
  let airdropSignature2 = await connection.requestAirdrop(
    pubkey,
    LAMPORTS_PER_SOL * 1000
  );
  await connection.confirmTransaction(airdropSignature2);
}
