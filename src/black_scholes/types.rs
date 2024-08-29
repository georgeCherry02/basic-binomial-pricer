use crate::option::FinancialOption;
use crate::result::{PricerError, PricerResult};
use crate::risk_factors::discount::{DiscountFactor, DiscountRf, InterestRate};
use crate::risk_factors::dividend::{AnnualisedDividendRate, Dividend};
use crate::risk_factors::price::{Price, PriceRf, PriceTick};
use crate::risk_factors::volatility::{ImpliedVolatility, Volatility, VolatilityRf};
use crate::risk_factors::{IdentifiableRiskFactor, RiskFactors};

use crate::shock::{ApplyShock, Scenario, Shock};
use crate::symbol::Symbol;

use crate::utils::date::get_duration_in_years;

use chrono::{DateTime, Utc};

pub struct BlackScholesRiskFactors {
    price_risk_factor: Price,
    volatility_risk_factor: Volatility,
    discounting_factor: DiscountFactor,
    dividend_factor: AnnualisedDividendRate,
}

impl BlackScholesRiskFactors {
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
            discounting_factor: DiscountFactor::RiskFreeRate(InterestRate::new(
                rfr_symbol,
                risk_free_rate,
            )),
            dividend_factor: AnnualisedDividendRate::new(symbol, dividend_rate),
        }
    }
}

pub struct BlackScholesInputs {
    pub delta_t: f64,
    risk_factors: BlackScholesRiskFactors,
}

impl BlackScholesInputs {
    pub fn gather<T: FinancialOption>(
        option: &T,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> BlackScholesInputs {
        let delta_t = get_duration_in_years(valuation_time, option.expiry());
        BlackScholesInputs {
            delta_t,
            risk_factors,
        }
    }
    pub fn discount_rate(&self) -> f64 {
        self.risk_factors.discounting_factor.rate()
    }
    pub fn annualised_dividend_rate(&self) -> f64 {
        self.risk_factors.dividend_factor.rate()
    }
    pub fn price(&self) -> f64 {
        self.risk_factors.price_risk_factor.price()
    }
    pub fn volatility(&self) -> f64 {
        self.risk_factors.volatility_risk_factor.volatility()
    }
    pub fn risk_free_adjustment(&self) -> f64 {
        self.risk_factors
            .discounting_factor
            .discount_factor(self.delta_t)
    }
    pub fn volatility_for_delta_t(&self) -> f64 {
        self.risk_factors
            .volatility_risk_factor
            .scaled_to_time(self.delta_t)
    }
    pub fn dividend_adjustment(&self) -> f64 {
        (-self.risk_factors.dividend_factor.rate() * self.delta_t).exp()
    }
    pub fn dividend_adjusted_price(&self) -> f64 {
        self.price() * self.dividend_adjustment()
    }
}

impl ApplyShock<BlackScholesInputs> for Shock {
    fn apply(&self, applicant: &mut BlackScholesInputs) {
        match self {
            Shock::InterestRateShock(shock) => {
                shock.apply(&mut applicant.risk_factors.discounting_factor)
            }
            Shock::PriceShock(shock) => shock.apply(&mut applicant.risk_factors.price_risk_factor),
            Shock::TimeShock(shock) => shock.apply(&mut applicant.delta_t),
            Shock::VolatilityShock(shock) => {
                shock.apply(&mut applicant.risk_factors.volatility_risk_factor)
            }
        }
    }
}

impl ApplyShock<BlackScholesInputs> for Scenario {
    fn apply(&self, applicant: &mut BlackScholesInputs) {
        for shock in self {
            shock.apply(applicant)
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
        let discounting_factor = get_first_and_ensure_one(risk_factors.discount_factors)?;
        let dividend_factor = get_first_and_ensure_one(risk_factors.dividend_sensitivities)
            .and_then(|dividend| match dividend {
            Dividend::AnnualisedRate(adr) => Ok(adr),
            Dividend::Schedule => Err(PricerError::new("Provided a dividend schedule to Black-Scholes, the pricer does not support this, please provide an annualised rate".into(), 5)),
        })?;
        Ok(BlackScholesRiskFactors {
            price_risk_factor,
            volatility_risk_factor,
            discounting_factor,
            dividend_factor,
        })
    }
}
