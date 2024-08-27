use super::pricing::BlackScholes;
use crate::option::{get_call, get_put, Call, Put};
use crate::risk_factor::RiskFactors;
use chrono::{DateTime, TimeZone, Utc};

fn get_test_evaluation_period() -> (DateTime<Utc>, DateTime<Utc>) {
    (
        Utc.timestamp_millis_opt(1688917143000).unwrap(),
        Utc.timestamp_millis_opt(1704697100000).unwrap(),
    )
}

fn get_test_risk_factors() -> RiskFactors {
    let underlying_price = 42f64;
    let underlying_volatility = 0.2f64;
    let risk_free_rate = 0.05f64;
    RiskFactors {
        underlying_price,
        underlying_volatility,
        risk_free_rate,
    }
}

fn get_test_inputs_call() -> (Call, DateTime<Utc>, RiskFactors) {
    let cost = 0f64;
    let strike = 40f64;
    let (begin_date, end_date) = get_test_evaluation_period();
    let call = get_call(strike, end_date, cost);
    let risk_factors = get_test_risk_factors();
    (call, begin_date, risk_factors)
}

fn get_test_inputs_put() -> (Put, DateTime<Utc>, RiskFactors) {
    let cost = 0f64;
    let strike = 40f64;
    let (begin_date, end_date) = get_test_evaluation_period();
    let put = get_put(strike, end_date, cost);
    let risk_factors = get_test_risk_factors();
    (put, begin_date, risk_factors)
}

#[test]
fn half_year_black_scholes_put() {
    let (put, begin_date, risk_factors) = get_test_inputs_put();
    #[allow(unused_must_use)]
    {
        put.value_black_scholes(begin_date, risk_factors, vec![])
            .map(|value| {
                assert!(value > 1.0934);
                assert!(value < 1.0935);
            });
    }
}

#[test]
fn half_year_black_scholes_call() {
    let (call, begin_date, risk_factors) = get_test_inputs_call();
    #[allow(unused_must_use)]
    {
        call.value_black_scholes(begin_date, risk_factors, vec![])
            .map(|value| {
                assert!(value > 4.0817);
                assert!(value < 4.0818);
            });
    }
}
