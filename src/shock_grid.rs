use pyo3::prelude::*;

use itertools::{Itertools, Product};

#[pyclass(frozen)]
pub struct ShockPoint {
    price: f64,
    volatility: f64,
}

#[pyclass(frozen)]
pub struct ShockGrid {
    shocks: Vec<ShockPoint>,
    dimensions: (u32, u32),
}

#[pyclass(frozen)]
pub struct ShockLimits {
    up: f64,
    down: f64,
    resolution: u32,
}

#[pymethods]
impl ShockLimits {
    #[new]
    pub fn new(up: f64, down: f64, resolution: u32) -> Self {
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

#[pyfunction]
pub fn generate_shock_grid(
    price: f64,
    price_shock_limits: Bound<ShockLimits>,
    volatility: f64,
    volatility_shock_limits: Bound<ShockLimits>,
) -> ShockGrid {
    let psl: &ShockLimits = price_shock_limits.get();
    let vsl: &ShockLimits = volatility_shock_limits.get();
    let price_step = get_step_size(price, psl);
    let vol_step = get_step_size(volatility, vsl);
    let price_begin = price * psl.down;
    let vol_begin = volatility * vsl.down;
    let prices = (0..psl.resolution).map(|s| price_begin + ((s as f64) * price_step));
    let vols = (0..vsl.resolution).map(|s| vol_begin + ((s as f64) * vol_step));
    let shocks = prices
        .cartesian_product(vols)
        .map(|(price, volatility)| ShockPoint { price, volatility })
        .collect();
    let dimensions = (psl.resolution, vsl.resolution);
    ShockGrid { shocks, dimensions }
}
