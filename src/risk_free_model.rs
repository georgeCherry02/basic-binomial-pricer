use chrono::prelude::Utc;
use chrono::DateTime;

use log::info;

pub trait RiskFreeModel {
    fn apply(&self, start_value: f64, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64;
}

pub struct AnnualisedRiskFreeRate {
    ir_exponent: f64,
}

pub fn get_annualised_risk_free_rate(apr: f64) -> Box<dyn RiskFreeModel> {
    let interest_rate = 1.0 + (apr / 100.0);
    info!("Constructed model with interest_rate={}", interest_rate);
    let ir_exponent = interest_rate.ln();
    Box::new(AnnualisedRiskFreeRate { ir_exponent })
}

const NUMBER_OF_SECONDS_IN_A_YEAR: f64 = 31536000.0;
const EULERS_NUMBER: f64 = std::f64::consts::E;

impl RiskFreeModel for AnnualisedRiskFreeRate {
    fn apply(&self, start_value: f64, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64 {
        info!("Hit apply");
        let diff_secs = end_date.signed_duration_since(start_date).num_seconds();
        let diff_years: f64 = (diff_secs as f64) / NUMBER_OF_SECONDS_IN_A_YEAR;
        info!("Diff years={}", diff_years);
        start_value * EULERS_NUMBER.powf(self.ir_exponent * diff_years)
    }
}
