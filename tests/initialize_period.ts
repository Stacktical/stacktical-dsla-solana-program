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

  const [periodGenerator, _bump] = await PublicKey.findProgramAddress(
    [anchor.utils.bytes.utf8.encode("period_generator"), owner.toBuffer()],
    program.programId
  );

  let systemProgram = SystemProgram.programId;
  let now = Date.now();
  // 5 min, 10 hour, 10 days
  let start = [
    new BN(now + 1000 * 60 * 5),
    new BN(now + 1000 * 60 * 60 * 10),
    new BN(now + 1000 * 60 * 60 * 24 * 10),
  ];
  // 30 sec, 1 day, 1 week
  let period_length = [
    new BN(1000 * 30),
    new BN(1000 * 60 * 60 * 24),
    new BN(1000 * 60 * 60 * 24 * 7),
  ];
  let n_periods = [new BN("0"), new BN("1"), new BN("100")];

  // TODO: some issue with getting the correct time on local chain
  // it("should fail with start too early ", async () => {
  //   try {
  //     await program.methods
  //       .initializePeriod(start[0], period_length[1], n_periods[1])
  //       .accounts({
  //         owner,
  //         periodGenerator,
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
        .initializePeriod(start[1], period_length[0], n_periods[1])
        .accounts({
          owner,
          periodGenerator,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      expect(false, "should return an error");
    } catch (e) {
      expect(e, "wrong error returned").to.have.property("programErrorStack");
    }
  });

  it("should fail with period = 0 ", async () => {
    try {
      await program.methods
        .initializePeriod(start[0], period_length[1], n_periods[0])
        .accounts({
          owner,
          periodGenerator,
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
      .initializePeriod(start[1], period_length[1], n_periods[1])
      .accounts({
        owner,
        periodGenerator,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let period_account = await program.account.periodGenerator.fetch(
      periodGenerator
    );

    expect(
      period_account.start.toNumber(),
      "period start does not match the input"
    ).to.equal(start[1].toNumber());

    expect(
      period_account.periodLength.toNumber(),
      "period length does not match the input"
    ).to.equal(period_length[1].toNumber());

    expect(
      period_account.nPeriods.toNumber(),
      "number of periods does not match the input"
    ).to.equal(n_periods[1].toNumber());
  });

  it("should fail to initialize the same Period for the same SLA twice", async () => {
    try {
      await program.methods
        .initializePeriod(start[2], period_length[2], n_periods[2])
        .accounts({
          owner,
          periodGenerator,
          systemProgram,
        })
        .rpc();
      expect(false, "should return an error");
    } catch (e) {
      expect(e, "wrong error returned").to.have.property("programErrorStack");
    }

    let period_account = await program.account.periodGenerator.fetch(
      periodGenerator
    );

    expect(
      period_account.start.toNumber(),
      "period start does not match the input"
    ).to.equal(start[1].toNumber());

    expect(
      period_account.periodLength.toNumber(),
      "period length does not match the input"
    ).to.equal(period_length[1].toNumber());

    expect(
      period_account.nPeriods.toNumber(),
      "number of periods does not match the input"
    ).to.equal(n_periods[1].toNumber());
  });
});
