use super::pricing::BlackScholes;
use crate::option::{get_call, get_put};
use crate::risk_factor::RiskFactors;
use chrono::{TimeZone, Utc};

#[test]
fn half_year_black_scholes_put() {
    let underlying_price = 42f64;
    let strike = 40f64;
    let cost = 0f64;
    let implied_volatility = 0.2f64;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1704697100000).unwrap();
    let put = get_put(strike, end_date, cost);
    let rfr = 0.05;
    let risk_factors = RiskFactors::new(underlying_price, implied_volatility, rfr);
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
    let underlying_price = 42f64;
    let strike = 40f64;
    let cost = 0f64;
    let implied_volatility = 0.2f64;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1704697100000).unwrap();
    let call = get_call(strike, end_date, cost);
    let rfr = 0.05;
    let risk_factors = RiskFactors::new(underlying_price, implied_volatility, rfr);
    #[allow(unused_must_use)]
    {
        call.value_black_scholes(begin_date, risk_factors, vec![])
            .map(|value| {
                assert!(value > 4.0817);
                assert!(value < 4.0818);
            });
    }
}
