use crate::result::{PricerError, PricerResult};

use crate::risk_factors::discount::{DiscountFactor, DiscountRf, InterestRate};
use crate::risk_factors::dividend::{AnnualisedDividendRate, Dividend};
use crate::risk_factors::price::{Price, PriceRf, PriceTick};
use crate::risk_factors::volatility::{ImpliedVolatility, Volatility, VolatilityRf};
use crate::risk_factors::{IdentifiableRiskFactor, RiskFactors};

use crate::shock::{ApplyShock, Shock};
use crate::symbol::Symbol;

pub struct BlackScholesRiskFactors {
    price_risk_factor: Price,
    volatility_risk_factor: Volatility,
    discount_factor: DiscountFactor,
    dividend_factor: AnnualisedDividendRate,
}

impl BlackScholesRiskFactors {
    pub fn discount_rate(&self) -> f64 {
        self.discount_factor.rate()
    }
    pub fn discount_factor(&self, delta_t: f64) -> f64 {
        self.discount_factor.discount_factor(delta_t)
    }
    pub fn annualised_dividend_rate(&self) -> f64 {
        self.dividend_factor.rate()
    }
    pub fn price(&self) -> f64 {
        self.price_risk_factor.price()
    }
    pub fn volatility(&self) -> f64 {
        self.volatility_risk_factor.volatility()
    }
    pub fn volatility_for_delta_t(&self, delta_t: f64) -> f64 {
        self.volatility_risk_factor.scaled_to_time(delta_t)
    }

    pub fn price_risk_factor(&self) -> &Symbol {
        self.price_risk_factor.id()
    }
    pub fn volatility_risk_factor(&self) -> &Symbol {
        self.volatility_risk_factor.id()
    }
    pub fn dividend_risk_factor(&self) -> &Symbol {
        self.dividend_factor.id()
    }
}

impl BlackScholesRiskFactors {
    pub fn new(
        symbol: Symbol,
        price: f64,
        volatility: f64,
        risk_free_rate: f64,
        rfr_symbol: Symbol,
        dividend_rate: f64,
    ) -> BlackScholesRiskFactors {
        BlackScholesRiskFactors {
            price_risk_factor: Price::PriceTick(PriceTick::new(symbol.clone(), price)),
            volatility_risk_factor: Volatility::ImpliedVolatility(ImpliedVolatility::new(
                symbol.clone(),
                volatility,
            )),
            discount_factor: DiscountFactor::RiskFreeRate(InterestRate::new(
                rfr_symbol,
                risk_free_rate,
            )),
            dividend_factor: AnnualisedDividendRate::new(symbol, dividend_rate),
        }
    }
}

fn too_many_rf_err(how_many: usize) -> PricerError {
    PricerError::new(
        format!("Provided {} risk factors, when 1 was expected", how_many),
        1,
    )
}
fn get_first_and_ensure_one<RF>(mut risk_factors: Vec<RF>) -> PricerResult<RF> {
    if risk_factors.len() != 1 {
        return Err(too_many_rf_err(risk_factors.len()));
    }
    Ok(risk_factors.remove(0))
}

impl TryFrom<RiskFactors> for BlackScholesRiskFactors {
    type Error = PricerError;
    fn try_from(risk_factors: RiskFactors) -> PricerResult<Self> {
        let price_risk_factor = get_first_and_ensure_one(risk_factors.price_sensitivities)?;
        let volatility_risk_factor =
            get_first_and_ensure_one(risk_factors.volatility_sensitivities)?;
        let discount_factor = get_first_and_ensure_one(risk_factors.discount_factors)?;
        let dividend_factor = get_first_and_ensure_one(risk_factors.dividend_sensitivities)
            .and_then(|dividend| match dividend {
            Dividend::AnnualisedRate(adr) => Ok(adr),
            Dividend::Schedule => Err(PricerError::new("Provided a dividend schedule to Black-Scholes, the pricer does not support this, please provide an annualised rate".into(), 5)),
        })?;
        Ok(BlackScholesRiskFactors {
            price_risk_factor,
            volatility_risk_factor,
            discount_factor,
            dividend_factor,
        })
    }
}

impl ApplyShock<BlackScholesRiskFactors> for Shock {
    fn apply(&self, applicant: &mut BlackScholesRiskFactors) {
        match self {
            Shock::InterestRateShock(shock) => shock.apply(&mut applicant.discount_factor),
            Shock::PriceShock(shock) => shock.apply(&mut applicant.price_risk_factor),
            Shock::VolatilityShock(shock) => shock.apply(&mut applicant.volatility_risk_factor),
            _ => (),
        }
    }
}
