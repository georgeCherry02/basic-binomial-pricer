use chrono::{DateTime, TimeZone, Utc};

use crate::option::{get_call, get_put, Call, Put};
use crate::risk_factors::RiskFactors;

fn get_test_evaluation_period() -> (DateTime<Utc>, DateTime<Utc>) {
    (
        Utc.timestamp_millis_opt(1688917143000).unwrap(),
        Utc.timestamp_millis_opt(1704697100000).unwrap(),
    )
}

fn get_test_risk_factors() -> RiskFactors {
    let underlying_price = 42.;
    let underlying_volatility = 0.2;
    let risk_free_rate = 0.05;
    let annualised_dividend_rate = 0.;
    let annualised_historic_return = 0.05;
    RiskFactors::new(
        underlying_price,
        underlying_volatility,
        risk_free_rate,
        annualised_dividend_rate,
        annualised_historic_return,
    )
}

pub fn get_test_inputs_call() -> (Call, DateTime<Utc>, RiskFactors) {
    let cost = 0f64;
    let strike = 40f64;
    let (begin_date, end_date) = get_test_evaluation_period();
    let call = get_call(strike, end_date, cost);
    let risk_factors = get_test_risk_factors();
    (call, begin_date, risk_factors)
}

pub fn get_test_inputs_put() -> (Put, DateTime<Utc>, RiskFactors) {
    let cost = 0f64;
    let strike = 40f64;
    let (begin_date, end_date) = get_test_evaluation_period();
    let put = get_put(strike, end_date, cost);
    let risk_factors = get_test_risk_factors();
    (put, begin_date, risk_factors)
}

pub fn is_close(lhs: f64, rhs: f64, percentage_tolerance: f64) -> bool {
    let magnitude = (lhs.abs() + rhs.abs()) / 2.;
    let difference = (rhs - lhs).abs();
    let perc_diff = difference / magnitude;
    println!("Percentage difference: {}", perc_diff);
    perc_diff < percentage_tolerance
}
