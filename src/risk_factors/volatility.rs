use super::IdentifiableRiskFactor;

use crate::shock::{ApplyShock, VolatilityShock};
use crate::symbol::Symbol;

use statrs::statistics::Statistics;

#[derive(Clone)]
pub struct ImpliedVolatility {
    symbol: Symbol,
    volatility: f64,
}

impl ImpliedVolatility {
    pub fn new(symbol: Symbol, volatility: f64) -> ImpliedVolatility {
        ImpliedVolatility { symbol, volatility }
    }
}

#[derive(Clone)]
pub struct HistoricVolatility {
    symbol: Symbol,
    price_time_series: Vec<f64>,
}

impl IdentifiableRiskFactor for ImpliedVolatility {
    fn id(&self) -> &Symbol {
        &self.symbol
    }
}

impl IdentifiableRiskFactor for HistoricVolatility {
    fn id(&self) -> &Symbol {
        &self.symbol
    }
}

pub trait VolatilityRf {
    fn volatility(&self) -> f64;
    fn scaled_to_time(&self, delta_t: f64) -> f64 {
        self.volatility() * delta_t.sqrt()
    }
}

impl VolatilityRf for ImpliedVolatility {
    fn volatility(&self) -> f64 {
        self.volatility
    }
}

impl VolatilityRf for HistoricVolatility {
    fn volatility(&self) -> f64 {
        self.price_time_series.iter().std_dev()
    }
}

#[derive(Clone)]
pub enum Volatility {
    ImpliedVolatility(ImpliedVolatility),
    HistoricVolatility(HistoricVolatility),
}

impl VolatilityRf for Volatility {
    fn volatility(&self) -> f64 {
        match &self {
            Volatility::ImpliedVolatility(iv) => iv.volatility(),
            Volatility::HistoricVolatility(hv) => hv.volatility(),
        }
    }
}

impl IdentifiableRiskFactor for Volatility {
    fn id(&self) -> &Symbol {
        match &self {
            Volatility::ImpliedVolatility(iv) => iv.id(),
            Volatility::HistoricVolatility(hv) => hv.id(),
        }
    }
}

impl ApplyShock<Volatility> for VolatilityShock {
    fn apply(&self, applicant: &mut Volatility) {
        match applicant {
            Volatility::ImpliedVolatility(iv) => self.apply(&mut iv.volatility),
            // This is also weird...
            Volatility::HistoricVolatility(_) => {}
        }
    }
}
