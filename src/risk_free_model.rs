use chrono::prelude::Utc;
use chrono::DateTime;
#[cfg(test)]
use chrono::TimeZone;

use log::debug;

pub trait RiskFreeModel {
    fn apply(&self, start_value: f64, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64;
}

pub struct AnnualisedRiskFreeRate {
    ir_exponent: f64,
}

pub fn get_annualised_risk_free_rate(apr: f64) -> Box<dyn RiskFreeModel> {
    let interest_rate = 1.0 + (apr / 100.0);
    debug!("Constructed model with interest_rate={}", interest_rate);
    let ir_exponent = interest_rate.ln();
    Box::new(AnnualisedRiskFreeRate { ir_exponent })
}

const NUMBER_OF_SECONDS_IN_A_YEAR: f64 = 31536000.0;
const EULERS_NUMBER: f64 = std::f64::consts::E;

impl RiskFreeModel for AnnualisedRiskFreeRate {
    fn apply(&self, start_value: f64, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64 {
        let diff_secs = end_date.signed_duration_since(start_date).num_seconds();
        let diff_years: f64 = (diff_secs as f64) / NUMBER_OF_SECONDS_IN_A_YEAR;
        start_value * EULERS_NUMBER.powf(self.ir_exponent * diff_years)
    }
}

#[test]
fn one_year_forward_test() {
    let rfm = get_annualised_risk_free_rate(5.0);
    let start: f64 = 100.0;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1720539543000).unwrap();
    let ret = rfm.apply(start, begin_date, end_date);
    let upper_bound: f64 = 105.02;
    let lower_bound: f64 = 105.01;
    assert!(ret < upper_bound);
    assert!(ret > lower_bound);
}

#[test]
fn one_year_backward_test() {
    let rfm = get_annualised_risk_free_rate(5.0);
    let start: f64 = 105.0;
    let begin_date = Utc.timestamp_millis_opt(1720539543000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let ret = rfm.apply(start, begin_date, end_date);
    let upper_bound: f64 = 99.99;
    let lower_bound: f64 = 99.98;
    assert!(ret < upper_bound);
    assert!(ret > lower_bound);
}
