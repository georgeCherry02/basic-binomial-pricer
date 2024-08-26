pub mod option;
pub mod result;
pub mod shock_grid;

mod black_scholes;
mod black_scholes_greeks;
mod greeks;
mod risk_factor;
mod shock;
mod utils;

use pyo3::prelude::*;

use chrono::prelude::Utc;

pub use black_scholes::BlackScholes;
use option::{Call, Put};
use risk_factor::RiskFactors;
use shock_grid::{generate_shock_grid, ShockGrid, ShockLimits};

use log::debug;

#[pyfunction]
pub fn price_black_scholes(
    py_call: Bound<Call>,
    volatility: f64,
    underlying_price: f64,
    apr: f64,
) -> PyResult<f64> {
    let call = py_call.borrow();
    let risk_factors = RiskFactors::new(underlying_price, volatility, apr);
    call.value_black_scholes(Utc::now(), risk_factors, vec![])
        .map_err(|e| e.into())
        .map(|r| {
            debug!("Valued call at {}", r);
            r
        })
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
    Ok(())
}
