import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { expect } from "chai";
import { Dsla } from "../target/types/dsla";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("initialize Period", async () => {
  const provider = anchor.AnchorProvider.local();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let owner = provider.wallet.publicKey;

  const program: Program<Dsla> = anchor.workspace.Dsla;

  const [periodRegistry, _bump] = await PublicKey.findProgramAddress(
    [anchor.utils.bytes.utf8.encode("period_registry"), owner.toBuffer()],
    program.programId
  );

  let systemProgram = SystemProgram.programId;

  let now = Date.now();
  // 5 min, 10 hour, 10 days
  let periods = [
    [
      { start: new BN(100000060000), end: new BN(1000000119999) },
      { start: new BN(1000000119999), end: new BN(1000000180000) },
      { start: new BN(1000000180000), end: new BN(1000000240000) },
    ],
    [
      { start: new BN(100000060000), end: new BN(1000000120000) },
      { start: new BN(1000000120000), end: new BN(1000000180000) },
      { start: new BN(1000000180000), end: new BN(1000000240000) },
    ],
    [
      { start: new BN(10000000000), end: new BN(20000000000) },
      { start: new BN(20000000000), end: new BN(30000000000) },
      { start: new BN(30000000000), end: new BN(40000000000) },
      { start: new BN(40000000000), end: new BN(50000000000) },
      { start: new BN(50000000000), end: new BN(60000000000) },
      { start: new BN(60000000000), end: new BN(70000000000) },
      { start: new BN(70000000000), end: new BN(80000000000) },
      { start: new BN(80000000000), end: new BN(90000000000) },
    ],
  ];

  // TODO: some issue with getting the correct time on local chain
  // it("should fail with start too early ", async () => {
  //   try {
  //     await program.methods
  //       .initializePeriodRegistry(start[0], period_length[1], n_periods[1])
  //       .accounts({
  //         owner,
  //         periodRegistry,
  //         systemProgram: SystemProgram.programId,
  //       })
  //       .rpc();
  //     expect(false, "should return an error");
  //   } catch (e) {
  //     console.log(e);
  //     expect(e, "wrong error returned").to.have.property("programErrorStack");
  //   }
  // });

  it("should fail with period length too short ", async () => {
    try {
      await program.methods
        .initializePeriodRegistry(periods[0])
        .accounts({
          owner,
          periodRegistry,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      expect(false, "should return an error");
    } catch (e) {
      expect(e, "wrong error returned").to.have.property("programErrorStack");
    }
  });

  it("should fail with length of periods too short ", async () => {
    try {
      await program.methods
        .initializePeriodRegistry([])
        .accounts({
          owner,
          periodRegistry,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      expect(false, "should return an error");
    } catch (e) {
      expect(e, "wrong error returned").to.have.property("programErrorStack");
    }
  });

  it("should succed to initialize the period", async () => {
    await program.methods
      .initializePeriodRegistry(periods[1])
      .accounts({
        owner,
        periodRegistry,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let period_account = await program.account.periodRegistry.fetch(
      periodRegistry
    );

    expect(
      period_account.periods,
      "period start does not match the input"
    ).to.equal(periods[1]);
  });

  it("should fail to initialize the same Period for the same SLA twice", async () => {
    try {
      await program.methods
        .initializePeriodRegistry(periods[2])
        .accounts({
          owner,
          periodRegistry,
          systemProgram,
        })
        .rpc();
      expect(false, "should return an error");
    } catch (e) {
      expect(e, "wrong error returned").to.have.property("programErrorStack");
    }

    let period_account = await program.account.periodRegistry.fetch(
      periodRegistry
    );

    expect(
      period_account.periods,
      "period start does not match the input"
    ).to.equal(periods[1]);
  });
});
