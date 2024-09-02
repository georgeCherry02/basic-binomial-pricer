use super::{LongstaffSchwartzMonteCarlo, MonteCarlo, MonteCarloParams};
use crate::black_scholes::BlackScholes;

use crate::result::PricerResult;
use crate::utils::test_utils::{get_test_call, get_test_ls_put, get_test_put, is_close};

fn monte_carlo_params() -> MonteCarloParams {
    MonteCarloParams {
        steps: 10000,
        repetitions: 1000,
    }
}

#[test]
fn half_year_call_monte_carlo_near_black_scholes() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_call();
    let black_scholes_valuation =
        call.value_black_scholes(valuation_time, risk_factors.clone(), vec![])?;
    let monte_carlo_valuation =
        call.value_monte_carlo(valuation_time, risk_factors, vec![], monte_carlo_params())?;
    assert!(
        is_close(black_scholes_valuation, monte_carlo_valuation, 0.15),
        "Monte Carlo valuation ({}) differs from Black-Scholes ({}) by more than 15%",
        monte_carlo_valuation,
        black_scholes_valuation
    );
    Ok(())
}

#[test]
fn half_year_put_monte_carlo_near_black_scholes() -> PricerResult<()> {
    let (put, valuation_time, risk_factors) = get_test_put();
    let black_scholes_valuation =
        put.value_black_scholes(valuation_time, risk_factors.clone(), vec![])?;
    let monte_carlo_valuation =
        put.value_monte_carlo(valuation_time, risk_factors, vec![], monte_carlo_params())?;
    assert!(
        is_close(black_scholes_valuation, monte_carlo_valuation, 0.15),
        "Monte Carlo valuation ({}) differs from Black-Scholes ({}) by more than 15%",
        monte_carlo_valuation,
        black_scholes_valuation
    );
    Ok(())
}

#[test]
fn direct_mcls_test() -> PricerResult<()> {
    let (put, valuation_time, risk_factors) = get_test_ls_put();
    let ls_valuation = put.value_monte_carlo_ls(
        valuation_time,
        risk_factors.clone(),
        vec![],
        monte_carlo_params(),
    )?;
    let expected = 3.28;
    assert!(
        is_close(ls_valuation, expected, 0.1),
        "Monte-Carlo Longstaff-Schwartz ({}) valuation differs from expected ({}) by more than 10%",
        ls_valuation,
        expected
    );
    Ok(())
}
