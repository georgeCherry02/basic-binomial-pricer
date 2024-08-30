use super::IdentifiableRiskFactor;

use crate::shock::{ApplyShock, PriceShock};
use crate::symbol::Symbol;

use statrs::statistics::Statistics;

#[derive(Clone)]
pub struct PriceTick {
    symbol: Symbol,
    price: f64,
}

impl PriceTick {
    pub fn new(symbol: Symbol, price: f64) -> PriceTick {
        PriceTick { symbol, price }
    }
}

#[derive(Clone)]
pub struct HistoricPrices {
    symbol: Symbol,
    historic_daily_average: Vec<f64>,
}

impl HistoricPrices {
    fn new(symbol: Symbol, historic_daily_average: Vec<f64>) -> HistoricPrices {
        HistoricPrices {
            symbol,
            historic_daily_average,
        }
    }
}

impl IdentifiableRiskFactor for PriceTick {
    fn id(&self) -> &Symbol {
        &self.symbol
    }
}

impl IdentifiableRiskFactor for HistoricPrices {
    fn id(&self) -> &Symbol {
        &self.symbol
    }
}

pub trait PriceRf {
    fn price(&self) -> f64;
}

impl PriceRf for PriceTick {
    fn price(&self) -> f64 {
        self.price
    }
}

impl PriceRf for HistoricPrices {
    fn price(&self) -> f64 {
        self.historic_daily_average.iter().mean()
    }
}

#[derive(Clone)]
pub enum Price {
    PriceTick(PriceTick),
    HistoricPrices(HistoricPrices),
}

impl PriceRf for Price {
    fn price(&self) -> f64 {
        match &self {
            Price::PriceTick(sp) => sp.price(),
            Price::HistoricPrices(hp) => hp.price(),
        }
    }
}

impl IdentifiableRiskFactor for Price {
    fn id(&self) -> &Symbol {
        match &self {
            Price::PriceTick(sp) => sp.id(),
            Price::HistoricPrices(hp) => hp.id(),
        }
    }
}

impl ApplyShock<Price> for PriceShock {
    fn apply(&self, applicant: &mut Price) {
        if applicant.id() != self.risk_factor() {
            return;
        }
        match applicant {
            Price::PriceTick(pt) => self.apply(&mut pt.price),
            // Uncertain how to model shocking historical price data
            Price::HistoricPrices(_) => {}
        }
    }
}
