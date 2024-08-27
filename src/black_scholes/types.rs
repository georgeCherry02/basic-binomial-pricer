use chrono::{DateTime, Utc};

use crate::option::FinancialOption;
use crate::risk_factor::RiskFactors;
use crate::utils::date::get_duration_in_years;

pub struct BlackScholesInputs {
    pub delta_t: f64,
    pub underlying_price: f64,
    pub underlying_volatility: f64,
    pub risk_free_rate: f64,
}

impl BlackScholesInputs {
    pub fn gather<T: FinancialOption>(
        option: &T,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> BlackScholesInputs {
        let delta_t = get_duration_in_years(valuation_time, option.expiry());
        BlackScholesInputs {
            delta_t,
            underlying_price: risk_factors.underlying_price,
            underlying_volatility: risk_factors.underlying_volatility,
            risk_free_rate: risk_factors.risk_free_rate,
        }
    }
    pub fn risk_free_adjustment(&self) -> f64 {
        (-self.delta_t * self.risk_free_rate).exp()
    }
}
