use crate::symbol::Symbol;

use statrs::statistics::Statistics;

const BUSINESS_DAYS_IN_YEAR: usize = 252;

pub struct HistoricReturn {
    symbol: Symbol,
    daily_yields: Vec<f64>,
}

pub struct InterestRate {
    symbol: Symbol,
    rate: f64,
}

pub trait DiscountRf {
    fn rate(&self) -> f64;
    fn discount_factor(&self, delta_t: f64) -> f64 {
        (-self.rate() * delta_t).exp()
    }
}

impl DiscountRf for HistoricReturn {
    fn rate(&self) -> f64 {
        self.daily_yields
            .windows(252)
            .map(|window| window.iter().sum::<f64>())
            .mean()
    }
}

impl DiscountRf for InterestRate {
    fn rate(&self) -> f64 {
        self.rate
    }
}

pub enum DiscountFactor {
    HistoricReturn(HistoricReturn),
    RiskFreeRate(InterestRate),
}

impl DiscountRf for DiscountFactor {
    fn rate(&self) -> f64 {
        match &self {
            DiscountFactor::RiskFreeRate(rfr) => rfr.rate(),
            DiscountFactor::HistoricReturn(hr) => hr.rate(),
        }
    }
}
