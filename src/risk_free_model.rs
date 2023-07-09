use chrono::prelude::Utc;
use chrono::DateTime;

pub trait RiskFreeModel {
    fn apply(&self, start_value: f64, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64;
}

pub struct AnnualisedRiskFreeRate {
    apr: f64,
}

const NUMBER_OF_SECONDS_IN_A_YEAR: f64 = 31536000.0;
const EULERS_NUMBER: f64 = std::f64::consts::E;

impl RiskFreeModel for AnnualisedRiskFreeRate {
    fn apply(&self, start_value: f64, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64 {
        let diff_secs = end_date.signed_duration_since(start_date).num_seconds();
        let diff_years: f64 = (diff_secs as f64) / NUMBER_OF_SECONDS_IN_A_YEAR;
        start_value * EULERS_NUMBER.powf(self.apr * diff_years)
    }
}
