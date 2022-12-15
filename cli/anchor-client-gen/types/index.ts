import * as FeedErrorCode from "./FeedErrorCode"
import * as SloType from "./SloType"
import * as PeriodLength from "./PeriodLength"
import * as SlaStatus from "./SlaStatus"
import * as Status from "./Status"

export { Slo } from "./Slo"
export type { SloFields, SloJSON } from "./Slo"
export { DslaDecimal } from "./DslaDecimal"
export type { DslaDecimalFields, DslaDecimalJSON } from "./DslaDecimal"
export { PeriodGenerator } from "./PeriodGenerator"
export type {
  PeriodGeneratorFields,
  PeriodGeneratorJSON,
} from "./PeriodGenerator"
export { FeedErrorCode }

export type FeedErrorCodeKind =
  | FeedErrorCode.InvalidSwitchboardAccount
  | FeedErrorCode.StaleFeed
  | FeedErrorCode.ConfidenceIntervalExceeded
export type FeedErrorCodeJSON =
  | FeedErrorCode.InvalidSwitchboardAccountJSON
  | FeedErrorCode.StaleFeedJSON
  | FeedErrorCode.ConfidenceIntervalExceededJSON

export { SloType }

/** what type of service level objective is this `Slo` */
export type SloTypeKind =
  | SloType.EqualTo
  | SloType.NotEqualTo
  | SloType.SmallerThan
  | SloType.SmallerOrEqualTo
  | SloType.GreaterThan
  | SloType.GreaterOrEqualTo
export type SloTypeJSON =
  | SloType.EqualToJSON
  | SloType.NotEqualToJSON
  | SloType.SmallerThanJSON
  | SloType.SmallerOrEqualToJSON
  | SloType.GreaterThanJSON
  | SloType.GreaterOrEqualToJSON

export { PeriodLength }

export type PeriodLengthKind =
  | PeriodLength.Custom
  | PeriodLength.Monthly
  | PeriodLength.Yearly
export type PeriodLengthJSON =
  | PeriodLength.CustomJSON
  | PeriodLength.MonthlyJSON
  | PeriodLength.YearlyJSON

export { SlaStatus }

/** The `SlaStatus` is in an enum to define the status of the `Sla` */
export type SlaStatusKind =
  | SlaStatus.NotStarted
  | SlaStatus.Active
  | SlaStatus.Ended
export type SlaStatusJSON =
  | SlaStatus.NotStartedJSON
  | SlaStatus.ActiveJSON
  | SlaStatus.EndedJSON

export { Status }

/** Enum defining the status */
export type StatusKind =
  | Status.NotVerified
  | Status.Respected
  | Status.NotRespected
export type StatusJSON =
  | Status.NotVerifiedJSON
  | Status.RespectedJSON
  | Status.NotRespectedJSON
