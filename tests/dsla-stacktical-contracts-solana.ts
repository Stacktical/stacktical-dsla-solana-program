import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { DslaStackticalContractsSolana } from '../target/types/dsla_stacktical_contracts_solana';

describe('dsla-stacktical-contracts-solana', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.DslaStackticalContractsSolana as Program<DslaStackticalContractsSolana>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
