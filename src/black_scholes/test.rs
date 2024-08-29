use super::BlackScholes;

use super::BlackScholesGreeks;

use crate::greeks::FiniteDifferenceGreeks;
use crate::result::PricerResult;
use crate::Priceable;

use crate::utils::test_utils::{get_test_inputs_call, get_test_inputs_put, is_close};

#[test]
#[allow(unused_must_use)]
fn half_year_black_scholes_put() {
    let (put, begin_date, risk_factors) = get_test_inputs_put();
    put.value(begin_date, risk_factors, vec![]).map(|value| {
        assert!(value > 1.0934);
        assert!(value < 1.0935);
    });
}

#[test]
#[allow(unused_must_use)]
fn half_year_black_scholes_call() {
    let (call, begin_date, risk_factors) = get_test_inputs_call();
    call.value(begin_date, risk_factors, vec![]).map(|value| {
        assert!(value > 4.0817);
        assert!(value < 4.0818);
    });
}

#[test]
#[allow(unused_must_use)]
fn black_scholes_finite_difference_delta_near_analytical_delta() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_inputs_call();
    let priceable = Priceable::BlackScholes(&call);
    let delta_finite_difference = priceable.delta_fd(valuation_time, risk_factors.clone())?;
    let delta_analytic = call.delta(valuation_time, risk_factors)?;
    assert!(is_close(delta_finite_difference, delta_analytic, 0.05), "Finite difference delta ({}) differs from analytical delta({}) for Black-Scholes by more than 5%", delta_finite_difference, delta_analytic);
    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn black_scholes_finite_difference_vega_near_analytical_vega() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_inputs_call();
    let priceable = Priceable::BlackScholes(&call);
    let vega_finite_difference = priceable.vega_fd(valuation_time, risk_factors.clone())?;
    let vega_analytic = call.vega(valuation_time, risk_factors)?;
    assert!(is_close(vega_finite_difference, vega_analytic, 0.1), "Finite difference vega ({}) differs from analytical vega({}) for Black-Scholes by more than 10%", vega_finite_difference, vega_analytic);
    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn black_scholes_finite_difference_rho_near_analytical_rho() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_inputs_call();
    let priceable = Priceable::BlackScholes(&call);
    let rho_finite_difference = priceable.rho_fd(valuation_time, risk_factors.clone())?;
    let rho_analytic = call.rho(valuation_time, risk_factors)?;
    assert!(
        is_close(rho_finite_difference, rho_analytic, 0.1),
        "Finite difference rho ({}) differs from analytical rho({}) for Black-Scholes by more than 10%",
        rho_finite_difference,
        rho_analytic
    );
    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn black_scholes_finite_difference_theta_near_analytical_theta() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_inputs_call();
    let priceable = Priceable::BlackScholes(&call);
    let theta_finite_difference = priceable.theta_fd(valuation_time, risk_factors.clone())?;
    let theta_analytic = call.theta(valuation_time, risk_factors)?;
    assert!(is_close(theta_finite_difference, theta_analytic, 0.05), "Finite difference theta ({}) differs from analytical theta({}) for Black-Scholes by more than 5%", theta_finite_difference, theta_analytic);
    Ok(())
}
