use super::BlackScholes;
use super::BlackScholesGreeks;
use crate::greeks::FiniteDifferenceGreeks;
use crate::option::{get_call, get_put, Call, Put};
use crate::result::PricerResult;
use crate::risk_factor::RiskFactors;
use chrono::{DateTime, TimeZone, Utc};
use statrs::assert_almost_eq;

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
    RiskFactors::new(underlying_price, underlying_volatility, risk_free_rate)
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
#[allow(unused_must_use)]
fn half_year_black_scholes_put() {
    let (put, begin_date, risk_factors) = get_test_inputs_put();
    put.value_black_scholes(begin_date, risk_factors, vec![])
        .map(|value| {
            assert!(value > 1.0934);
            assert!(value < 1.0935);
        });
}

#[test]
#[allow(unused_must_use)]
fn half_year_black_scholes_call() {
    let (call, begin_date, risk_factors) = get_test_inputs_call();
    call.value_black_scholes(begin_date, risk_factors, vec![])
        .map(|value| {
            assert!(value > 4.0817);
            assert!(value < 4.0818);
        });
}

fn is_close(lhs: f64, rhs: f64, tolerance: f64) -> bool {
    let approximate_magnitude = (lhs.abs() + rhs.abs()) / 2.0;
    let abs_difference = (rhs - lhs).abs();
    (abs_difference / approximate_magnitude) < tolerance
}

#[test]
#[allow(unused_must_use)]
fn black_scholes_finite_difference_delta_near_analytical_delta() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_inputs_call();
    let delta_finite_difference = call.delta_fd(valuation_time, risk_factors.clone())?;
    let delta_analytic = call.delta(valuation_time, risk_factors)?;
    assert_almost_eq!(delta_finite_difference, delta_analytic, 0.05);
    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn black_scholes_finite_difference_vega_near_analytical_vega() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_inputs_call();
    let delta_finite_difference = call.vega_fd(valuation_time, risk_factors.clone())?;
    let delta_analytic = call.vega(valuation_time, risk_factors)?;
    assert_almost_eq!(delta_finite_difference, delta_analytic, 0.05);
    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn black_scholes_finite_difference_rho_near_analytical_rho() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_inputs_call();
    let delta_finite_difference = call.rho_fd(valuation_time, risk_factors.clone())?;
    let delta_analytic = call.rho(valuation_time, risk_factors)?;
    assert_almost_eq!(delta_finite_difference, delta_analytic, 0.05);
    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn black_scholes_finite_difference_theta_near_analytical_theta() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_inputs_call();
    let delta_finite_difference = call.theta_fd(valuation_time, risk_factors.clone())?;
    let delta_analytic = call.theta(valuation_time, risk_factors)?;
    assert_almost_eq!(delta_finite_difference, delta_analytic, 0.05);
    Ok(())
}
