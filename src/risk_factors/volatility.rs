use crate::symbol::Symbol;

use statrs::statistics::Statistics;

pub struct ImpliedVolatility {
    symbol: Symbol,
    volatility: f64,
}

pub struct HistoricVolatility {
    symbol: Symbol,
    price_time_series: Vec<f64>,
}

pub trait VolatilityRf {
    fn volatility(&self) -> f64;
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
