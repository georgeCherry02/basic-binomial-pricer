use crate::symbol::Symbol;

use statrs::statistics::Statistics;

pub struct PriceTick {
    symbol: Symbol,
    price: f64,
}

pub struct HistoricPrices {
    symbol: Symbol,
    historic_daily_average: Vec<f64>,
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
