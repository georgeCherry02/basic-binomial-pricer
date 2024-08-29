use crate::black_scholes::BlackScholes;
use crate::option::Call;
use crate::risk_factors::discount::rfr_discount;

use pyo3::prelude::*;

use chrono::Utc;
use itertools::Itertools;

#[pyclass(frozen)]
pub struct ShockPoint {
    price: f64,
    volatility: f64,
}

#[pyclass(frozen)]
pub struct ShockGrid {
    shocks: Vec<ShockPoint>,
    dimensions: (usize, usize),
}

#[pymethods]
impl ShockGrid {
    fn prices(&self) -> Vec<f64> {
        (0..self.dimensions.1)
            .map(|i| i * self.dimensions.0)
            .map(|i| self.shocks[i].price)
            .collect()
    }
    fn volatilities(&self) -> Vec<f64> {
        self.shocks
            .iter()
            .map(|sp| sp.volatility)
            .take(self.dimensions.0)
            .collect()
    }
    fn value_black_scholes(&self, py_call: Bound<Call>, risk_free_rate: f64) -> Vec<Vec<f64>> {
        let call: &Call = py_call.get();
        let now = Utc::now();
        let discounting_factor = rfr_discount("US Treasury 3M".into(), risk_free_rate);
        let valuations = self.shocks.iter().map(|shock_point| {
            let risk_factors = call.get_risk_factors(
                shock_point.price,
                shock_point.volatility,
                0.,
                discounting_factor.clone(),
            );
            call.value(now, risk_factors, vec![]).unwrap_or_default()
        });
        let (n_price, n_vol) = self.dimensions;
        let mut out = vec![Vec::with_capacity(n_price); n_vol];
        for (i, valuation) in valuations.enumerate() {
            out[i % n_vol].push(valuation);
        }
        out
    }
}

#[pyclass(frozen)]
pub struct ShockLimits {
    up: f64,
    down: f64,
    resolution: usize,
}

#[pymethods]
impl ShockLimits {
    #[new]
    pub fn new(up: f64, down: f64, resolution: usize) -> Self {
        ShockLimits {
            up,
            down,
            resolution,
        }
    }
}

fn get_step_size(shocked_element: f64, shock_limits: &ShockLimits) -> f64 {
    let total_distance = shocked_element * (shock_limits.up + shock_limits.down);
    total_distance / (shock_limits.resolution as f64)
}

fn generate_shock_iter(
    shocked_element: f64,
    shock_limits: &ShockLimits,
) -> impl Iterator<Item = f64> {
    let step_size = get_step_size(shocked_element, shock_limits);
    let begin = shocked_element - shocked_element * shock_limits.down;
    (0..shock_limits.resolution).map(move |step| begin + ((step as f64) * step_size))
}

#[pyfunction]
pub fn generate_shock_grid(
    price: f64,
    price_shock_limits: Bound<ShockLimits>,
    volatility: f64,
    volatility_shock_limits: Bound<ShockLimits>,
) -> ShockGrid {
    let psl: &ShockLimits = price_shock_limits.get();
    let vsl: &ShockLimits = volatility_shock_limits.get();
    let prices = generate_shock_iter(price, psl);
    let vols = generate_shock_iter(volatility, vsl).collect::<Vec<f64>>();
    let shocks = prices
        .cartesian_product(vols)
        .map(|(price, volatility)| ShockPoint { price, volatility })
        .collect();
    let dimensions = (vsl.resolution, psl.resolution);
    ShockGrid { shocks, dimensions }
}
