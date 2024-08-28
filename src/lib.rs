pub mod option;
pub mod result;
pub mod shock_grid;

mod black_scholes;
mod monte_carlo;

mod greeks;
mod risk_factor;
mod shock;
mod utils;

use pyo3::prelude::*;

use chrono::prelude::Utc;

pub use black_scholes::BlackScholes;
use monte_carlo::{generate_monte_carlo_paths, MonteCarloInputs, MonteCarloParams};
use option::{Call, FinancialOption, Put};
use risk_factor::RiskFactors;
use shock_grid::{generate_shock_grid, ShockGrid, ShockLimits};

use log::debug;
use utils::date::get_duration_in_years;

#[pyfunction]
pub fn price_black_scholes(
    py_call: Bound<Call>,
    volatility: f64,
    underlying_price: f64,
    apr: f64,
    dividend_rate: f64,
) -> PyResult<f64> {
    let call = py_call.borrow();
    let risk_factors = RiskFactors::new(underlying_price, volatility, apr, dividend_rate, 0.);
    call.value_black_scholes(Utc::now(), risk_factors, vec![])
        .map_err(|e| e.into())
        .map(|r| {
            debug!("Valued call at {}", r);
            r
        })
}

#[pyfunction]
pub fn gen_monte_carlo_paths(
    py_call: Bound<Call>,
    underlying_price: f64,
    underlying_volatility: f64,
    annualised_historic_return: f64,
) -> PyResult<Vec<Vec<f64>>> {
    let call = py_call.borrow();
    let delta_t = get_duration_in_years(Utc::now(), call.expiry());
    let inputs = MonteCarloInputs {
        delta_t,
        underlying_price,
        underlying_volatility,
        annualised_historic_return,
    };
    let params = MonteCarloParams {
        steps: 10000,
        repetitions: 1000,
    };
    generate_monte_carlo_paths(inputs, params).map_err(|e| e.into())
}

#[pymodule]
fn pricer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(price_black_scholes, m)?)?;
    m.add_class::<Put>()?;
    m.add_class::<Call>()?;

    m.add_class::<ShockGrid>()?;
    m.add_class::<ShockLimits>()?;
    m.add_function(wrap_pyfunction!(generate_shock_grid, m)?)?;

    m.add_function(wrap_pyfunction!(gen_monte_carlo_paths, m)?)?;
    Ok(())
}
