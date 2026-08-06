#[derive(Debug, Clone, PartialEq)]
pub enum RadarEquationsErrors {
    InvalidDetectionProbability,
    InvalidFalseAlarmProbability,
    InvalidPulseCount,
    IsInvalid,
}