pub mod option;
pub mod result;
pub mod shock_grid;

mod black_scholes;
mod monte_carlo;
mod tree;

mod finite_difference;
mod greeks;

mod risk_factors;
mod shock;
mod symbol;
mod utils;

use pyo3::prelude::*;

use chrono::{DateTime, Utc};

pub use black_scholes::{BlackScholes, BlackScholesRiskFactors};
use monte_carlo::{MonteCarlo, MonteCarloParams};

use option::{Call, Put};
use risk_factors::{discount::rfr_discount, RiskFactors};
use shock_grid::{generate_shock_grid, ShockGrid, ShockLimits};

use result::PricerResult;
use shock::Scenario;

use log::debug;

pub enum Priceable<'a> {
    BlackScholes(&'a dyn BlackScholes),
    MonteCarlo(&'a dyn MonteCarlo),
}

pub trait Pricer {
    fn value(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        scenario: Scenario,
    ) -> PricerResult<f64>;
}

impl Pricer for Priceable<'_> {
    fn value(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        scenario: Scenario,
    ) -> PricerResult<f64> {
        match &self {
            Priceable::BlackScholes(bs_option) => {
                bs_option.value_black_scholes(valuation_time, risk_factors, scenario)
            }
            Priceable::MonteCarlo(ms_option) => ms_option.value_monte_carlo(
                valuation_time,
                risk_factors,
                scenario,
                MonteCarloParams {
                    steps: 1000,
                    repetitions: 100,
                },
            ),
        }
    }
}

#[pyfunction]
pub fn price_black_scholes(
    py_call: Bound<Call>,
    volatility: f64,
    underlying_price: f64,
    apr: f64,
    dividend_rate: f64,
) -> PyResult<f64> {
    let call = py_call.borrow();
    let discounting_factor = rfr_discount("US Treasury 3M".into(), apr);
    let risk_factors = call.get_black_scholes_risk_factors(
        underlying_price,
        volatility,
        dividend_rate,
        discounting_factor,
    );
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
    let risk_factors = call.get_monte_carlo_risk_factors(
        underlying_price,
        underlying_volatility,
        annualised_historic_return,
    );
    call.generate_monte_carlo_paths(
        Utc::now(),
        risk_factors,
        MonteCarloParams {
            steps: 10000,
            repetitions: 1000,
        },
    )
    .map_err(|e| e.into())
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
