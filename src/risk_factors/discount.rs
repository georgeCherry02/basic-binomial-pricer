use crate::shock::{ApplyShock, InterestRateShock};
use crate::symbol::Symbol;

use statrs::statistics::Statistics;

use super::IdentifiableRiskFactor;

const BUSINESS_DAYS_IN_YEAR: usize = 252;

#[derive(Clone)]
pub struct HistoricReturnSeries {
    symbol: Symbol,
    daily_yields: Vec<f64>,
}

#[derive(Clone)]
pub struct HistoricReturn {
    symbol: Symbol,
    historic_return: f64,
}

#[derive(Clone)]
pub struct InterestRate {
    symbol: Symbol,
    rate: f64,
}

impl InterestRate {
    pub fn new(symbol: Symbol, rate: f64) -> InterestRate {
        InterestRate { symbol, rate }
    }
}

pub trait DiscountRf {
    fn rate(&self) -> f64;
    fn discount_factor(&self, delta_t: f64) -> f64 {
        (-self.rate() * delta_t).exp()
    }
}

impl DiscountRf for HistoricReturnSeries {
    fn rate(&self) -> f64 {
        self.daily_yields
            .windows(252)
            .map(|window| window.iter().sum::<f64>())
            .mean()
    }
}

impl IdentifiableRiskFactor for HistoricReturnSeries {
    fn id(&self) -> &Symbol {
        &self.symbol
    }
}

impl DiscountRf for InterestRate {
    fn rate(&self) -> f64 {
        self.rate
    }
}

impl IdentifiableRiskFactor for InterestRate {
    fn id(&self) -> &Symbol {
        &self.symbol
    }
}

impl DiscountRf for HistoricReturn {
    fn rate(&self) -> f64 {
        self.historic_return
    }
}

impl IdentifiableRiskFactor for HistoricReturn {
    fn id(&self) -> &Symbol {
        &self.symbol
    }
}

#[derive(Clone)]
pub enum DiscountFactor {
    HistoricReturnSeries(HistoricReturnSeries),
    HistoricReturn(HistoricReturn),
    RiskFreeRate(InterestRate),
}

impl DiscountRf for DiscountFactor {
    fn rate(&self) -> f64 {
        match &self {
            DiscountFactor::RiskFreeRate(rfr) => rfr.rate(),
            DiscountFactor::HistoricReturn(hr) => hr.rate(),
            DiscountFactor::HistoricReturnSeries(hr) => hr.rate(),
        }
    }
}

impl IdentifiableRiskFactor for DiscountFactor {
    fn id(&self) -> &Symbol {
        match &self {
            DiscountFactor::RiskFreeRate(rfr) => rfr.id(),
            DiscountFactor::HistoricReturn(hr) => hr.id(),
            DiscountFactor::HistoricReturnSeries(hr) => hr.id(),
        }
    }
}

impl ApplyShock<DiscountFactor> for InterestRateShock {
    fn apply(&self, applicant: &mut DiscountFactor) {
        match applicant {
            DiscountFactor::RiskFreeRate(rfr) => self.apply(&mut rfr.rate),
            // This is weird and not sure how it would interact?
            DiscountFactor::HistoricReturn(_) => {}
            DiscountFactor::HistoricReturnSeries(_) => {}
        }
    }
}

pub fn rfr_discount(symbol: Symbol, rate: f64) -> DiscountFactor {
    DiscountFactor::RiskFreeRate(InterestRate { symbol, rate })
}
