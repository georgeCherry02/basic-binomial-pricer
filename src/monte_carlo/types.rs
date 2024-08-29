use crate::option::FinancialOption;
use crate::result::{PricerError, PricerResult};
use crate::risk_factors::discount::{DiscountFactor, DiscountRf};
use crate::risk_factors::price::{Price, PriceRf};
use crate::risk_factors::volatility::{Volatility, VolatilityRf};
use crate::risk_factors::RiskFactors;
use crate::utils::date::get_duration_in_years;

use chrono::{DateTime, Utc};

pub struct MonteCarloRiskFactors {
    price_risk_factor: Price,
    volatility_risk_factor: Volatility,
    discount_factor: DiscountFactor,
}

pub struct MonteCarloInputs {
    pub delta_t: f64,
    risk_factors: MonteCarloRiskFactors,
}

impl MonteCarloInputs {
    pub fn gather(
        expiry: DateTime<Utc>,
        valuation_time: DateTime<Utc>,
        risk_factors: MonteCarloRiskFactors,
    ) -> MonteCarloInputs {
        let delta_t = get_duration_in_years(valuation_time, expiry);
        MonteCarloInputs {
            delta_t,
            risk_factors,
        }
    }

    pub fn discount_rate(&self) -> f64 {
        self.risk_factors.discount_factor.rate()
    }
    pub fn price(&self) -> f64 {
        self.risk_factors.price_risk_factor.price()
    }
    pub fn volatility(&self) -> f64 {
        self.risk_factors.volatility_risk_factor.volatility()
    }
    pub fn discount(&self, value: f64) -> f64 {
        value * (-self.delta_t * self.risk_factors.discount_factor.rate()).exp()
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

impl TryFrom<RiskFactors> for MonteCarloRiskFactors {
    type Error = PricerError;
    fn try_from(risk_factors: RiskFactors) -> PricerResult<Self> {
        let price_risk_factor = get_first_and_ensure_one(risk_factors.price_sensitivities)?;
        let volatility_risk_factor =
            get_first_and_ensure_one(risk_factors.volatility_sensitivities)?;
        let discount_factor = get_first_and_ensure_one(risk_factors.discount_factors)?;
        Ok(MonteCarloRiskFactors {
            price_risk_factor,
            volatility_risk_factor,
            discount_factor,
        })
    }
}

pub struct MonteCarloParams {
    pub steps: u64,
    pub repetitions: u64,
}
