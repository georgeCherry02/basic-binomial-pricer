use crate::greeks::FiniteDifferenceGreeks;
use crate::Priceable;
use crate::result::PricerResult;
use crate::utils::test_utils::{get_test_call, is_close};

#[test]
fn monte_carlo_near_black_scholes_finite_difference_delta() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_call();
    let black_scholes_priceable = Priceable::BlackScholes(&call);
    let monte_carlo_priceable = Priceable::MonteCarlo(&call);
    let black_scholes_delta = black_scholes_priceable.delta_fd(valuation_time, risk_factors.clone())?;
    let monte_carlo_delta = monte_carlo_priceable.delta_fd(valuation_time, risk_factors)?;
    assert!(
        is_close(black_scholes_delta, monte_carlo_delta, 0.2),
        "Black-Scholes finite difference delta ({}) differs from Monte Carlo ({}) by more than 20%",
        black_scholes_delta,
        monte_carlo_delta,
    );
    Ok(())
}

#[test]
fn monte_carlo_near_black_scholes_finite_difference_vega() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_call();
    let black_scholes_priceable = Priceable::BlackScholes(&call);
    let monte_carlo_priceable = Priceable::MonteCarlo(&call);
    let black_scholes_vega = black_scholes_priceable.vega_fd(valuation_time, risk_factors.clone())?;
    let monte_carlo_vega = monte_carlo_priceable.vega_fd(valuation_time, risk_factors)?;
    assert!(
        is_close(black_scholes_vega, monte_carlo_vega, 0.2),
        "Black-Scholes finite difference vega ({}) differs from Monte Carlo ({}) by more than 20%",
        black_scholes_vega,
        monte_carlo_vega,
    );
    Ok(())
}


#[test]
fn monte_carlo_near_black_scholes_finite_difference_rho() -> PricerResult<()> {
    let (call, valuation_time, risk_factors) = get_test_call();
    let black_scholes_priceable = Priceable::BlackScholes(&call);
    let monte_carlo_priceable = Priceable::MonteCarlo(&call);
    let black_scholes_rho = black_scholes_priceable.rho_fd(valuation_time, risk_factors.clone())?;
    let monte_carlo_rho = monte_carlo_priceable.rho_fd(valuation_time, risk_factors)?;
    assert!(
        is_close(black_scholes_rho, monte_carlo_rho, 0.2),
        "Black-Scholes finite difference rho ({}) differs from Monte Carlo ({}) by more than 20%",
        black_scholes_rho,
        monte_carlo_rho,
    );
    Ok(())
}
