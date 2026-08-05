#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RadarEquationsErrors {
    InvalidDetectionProbability,
    InvalidFalseAlarmProbability,
    InvalidPulseCount,
    IsInvalid,
}