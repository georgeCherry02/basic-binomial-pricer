pub mod discount;
pub mod dividend;
pub mod price;
pub mod volatility;

use crate::symbol::Symbol;

use discount::DiscountFactor;
use dividend::Dividend;
use price::Price;
use volatility::Volatility;

pub trait IdentifiableRiskFactor {
    fn id(&self) -> &Symbol;
}

#[derive(Clone)]
pub struct RiskFactors {
    pub price_sensitivities: Vec<Price>,
    pub volatility_sensitivities: Vec<Volatility>,
    pub discount_factors: Vec<DiscountFactor>,
    pub dividend_sensitivities: Vec<Dividend>,
}
