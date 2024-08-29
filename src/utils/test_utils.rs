use crate::black_scholes::BlackScholes;
use crate::monte_carlo::MonteCarlo;
use crate::option::{get_call, get_put, Call, Put};
use crate::risk_factors::discount::rfr_discount;
use crate::risk_factors::RiskFactors;
use crate::symbol::Symbol;

use chrono::{DateTime, TimeZone, Utc};
use std::convert::From;

fn get_test_evaluation_period() -> (DateTime<Utc>, DateTime<Utc>) {
    (
        Utc.timestamp_millis_opt(1688917143000).unwrap(),
        Utc.timestamp_millis_opt(1704697100000).unwrap(),
    )
}

fn get_test_risk_factors<T: BlackScholes>(option: &T) -> RiskFactors {
    let underlying_price = 42.;
    let underlying_volatility = 0.2;
    // Approximately 5%
    let treasury_symbol = Symbol::from("US Treasury 3M");
    let discount_rate = rfr_discount(treasury_symbol, 0.05);
    let annualised_dividend_rate = 0.;
    option.get_black_scholes_risk_factors(
        underlying_price,
        underlying_volatility,
        annualised_dividend_rate,
        discount_rate,
    )
}

pub fn get_test_call() -> (Call, DateTime<Utc>, RiskFactors) {
    let symbol = Symbol::from("AAPL");
    let cost = 0f64;
    let strike = 40f64;
    let (begin_date, end_date) = get_test_evaluation_period();
    let call = get_call(symbol.clone(), strike, end_date, cost);
    let risk_factors = get_test_risk_factors(&call);
    (call, begin_date, risk_factors)
}

pub fn get_test_put() -> (Put, DateTime<Utc>, RiskFactors) {
    let symbol = Symbol::from("AAPL");
    let cost = 0f64;
    let strike = 40f64;
    let (begin_date, end_date) = get_test_evaluation_period();
    let put = get_put(symbol.clone(), strike, end_date, cost);
    let risk_factors = get_test_risk_factors(&put);
    (put, begin_date, risk_factors)
}

pub fn is_close(lhs: f64, rhs: f64, percentage_tolerance: f64) -> bool {
    let magnitude = (lhs.abs() + rhs.abs()) / 2.;
    let difference = (rhs - lhs).abs();
    let perc_diff = difference / magnitude;
    println!("Percentage difference: {}", perc_diff);
    perc_diff < percentage_tolerance
}
