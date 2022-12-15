export type CustomError =
  | InvalidPrecision
  | InvalidPeriodId
  | AlreadyVerifiedPeriod
  | DecimalConversionError
  | CheckedOperationOverflow
  | NoAvailableTokensForWithdrawal
  | CannotStakeAfterSlaEnded
  | WithdrawalIsZero
  | SLaAlreadyInitialized
  | NonValidGovernanceParameters
  | SlaNotStarted

export class InvalidPrecision extends Error {
  static readonly code = 6000
  readonly code = 6000
  readonly name = "InvalidPrecision"
  readonly msg = "precision is not divisible by 100"

  constructor(readonly logs?: string[]) {
    super("6000: precision is not divisible by 100")
  }
}

export class InvalidPeriodId extends Error {
  static readonly code = 6001
  readonly code = 6001
  readonly name = "InvalidPeriodId"
  readonly msg = "period ID entered is not valid"

  constructor(readonly logs?: string[]) {
    super("6001: period ID entered is not valid")
  }
}

export class AlreadyVerifiedPeriod extends Error {
  static readonly code = 6002
  readonly code = 6002
  readonly name = "AlreadyVerifiedPeriod"
  readonly msg = "trying to verify an already verified period"

  constructor(readonly logs?: string[]) {
    super("6002: trying to verify an already verified period")
  }
}

export class DecimalConversionError extends Error {
  static readonly code = 6003
  readonly code = 6003
  readonly name = "DecimalConversionError"
  readonly msg = "Failed to convert to a decimal"

  constructor(readonly logs?: string[]) {
    super("6003: Failed to convert to a decimal")
  }
}

export class CheckedOperationOverflow extends Error {
  static readonly code = 6004
  readonly code = 6004
  readonly name = "CheckedOperationOverflow"
  readonly msg = "operation failed with an overflow"

  constructor(readonly logs?: string[]) {
    super("6004: operation failed with an overflow")
  }
}

export class NoAvailableTokensForWithdrawal extends Error {
  static readonly code = 6005
  readonly code = 6005
  readonly name = "NoAvailableTokensForWithdrawal"
  readonly msg = "Not enough available tokens for withdrawal"

  constructor(readonly logs?: string[]) {
    super("6005: Not enough available tokens for withdrawal")
  }
}

export class CannotStakeAfterSlaEnded extends Error {
  static readonly code = 6006
  readonly code = 6006
  readonly name = "CannotStakeAfterSlaEnded"
  readonly msg = "Cannot Stake After SLA has ended"

  constructor(readonly logs?: string[]) {
    super("6006: Cannot Stake After SLA has ended")
  }
}

export class WithdrawalIsZero extends Error {
  static readonly code = 6007
  readonly code = 6007
  readonly name = "WithdrawalIsZero"
  readonly msg = "Withdrawal should be at least 1"

  constructor(readonly logs?: string[]) {
    super("6007: Withdrawal should be at least 1")
  }
}

export class SLaAlreadyInitialized extends Error {
  static readonly code = 6008
  readonly code = 6008
  readonly name = "SLaAlreadyInitialized"
  readonly msg = "SLA with the same address can only be initialized once"

  constructor(readonly logs?: string[]) {
    super("6008: SLA with the same address can only be initialized once")
  }
}

export class NonValidGovernanceParameters extends Error {
  static readonly code = 6009
  readonly code = 6009
  readonly name = "NonValidGovernanceParameters"
  readonly msg = "1 or more non Valid governance Parameters"

  constructor(readonly logs?: string[]) {
    super("6009: 1 or more non Valid governance Parameters")
  }
}

export class SlaNotStarted extends Error {
  static readonly code = 6010
  readonly code = 6010
  readonly name = "SlaNotStarted"
  readonly msg = "Sla not started yet"

  constructor(readonly logs?: string[]) {
    super("6010: Sla not started yet")
  }
}

export function fromCode(code: number, logs?: string[]): CustomError | null {
  switch (code) {
    case 6000:
      return new InvalidPrecision(logs)
    case 6001:
      return new InvalidPeriodId(logs)
    case 6002:
      return new AlreadyVerifiedPeriod(logs)
    case 6003:
      return new DecimalConversionError(logs)
    case 6004:
      return new CheckedOperationOverflow(logs)
    case 6005:
      return new NoAvailableTokensForWithdrawal(logs)
    case 6006:
      return new CannotStakeAfterSlaEnded(logs)
    case 6007:
      return new WithdrawalIsZero(logs)
    case 6008:
      return new SLaAlreadyInitialized(logs)
    case 6009:
      return new NonValidGovernanceParameters(logs)
    case 6010:
      return new SlaNotStarted(logs)
  }

  return null
}
