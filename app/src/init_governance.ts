import { initGovernance } from '../../anchor-client-gen/instructions';
import { PROGRAM_ID } from '../../anchor-client-gen/programId';
import { SLA_PROTOCOL_DEPLOYER, GOVERNANCE_PARAMETERS, GOVERNANCE_SEED } from './constants';
import { PublicKey, Transaction, SystemProgram, Connection, sendAndConfirmTransaction } from '@solana/web3.js';
import { Governance } from '../../anchor-client-gen/accounts';

export function initGovernanceTx(connection: Connection) {
  // call an instruction
  const tx = new Transaction();

  const programDataPda = PublicKey.findProgramAddressSync(
    [PROGRAM_ID.toBuffer()],
    new PublicKey('BPFLoaderUpgradeab1e11111111111111111111111'),
  )[0];

  const goveranncePda = PublicKey.findProgramAddressSync([Buffer.from(GOVERNANCE_SEED)], PROGRAM_ID)[0];

  const ix = initGovernance(
    { ...GOVERNANCE_PARAMETERS },
    {
      programUpgradeAuthority: SLA_PROTOCOL_DEPLOYER.publicKey,
      governance: goveranncePda,
      program: PROGRAM_ID,
      programData: programDataPda,
      systemProgram: SystemProgram.programId,
    },
  );
  tx.add(ix);

  sendAndConfirmTransaction(connection, tx, [SLA_PROTOCOL_DEPLOYER]);
}

export async function fetch_governance_account(connection: Connection) {
  // fetch an account
  const addr = PublicKey.findProgramAddressSync([Buffer.from(GOVERNANCE_SEED)], PROGRAM_ID)[0];
  const acc = await Governance.fetch(connection, addr);
  if (acc === null) {
    // the fetch method returns null when the account is uninitialized
    console.log('account not found');
  }
  // convert to a JSON object
  const obj = acc.toJSON();
  return obj;
}
