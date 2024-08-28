use crate::option::FinancialOption;
use crate::risk_factor::RiskFactors;
use crate::utils::date::get_duration_in_years;

use chrono::{DateTime, Utc};

pub struct MonteCarloInputs {
    pub delta_t: f64,
    pub underlying_price: f64,
    pub underlying_volatility: f64,
    pub annualised_historic_return: f64,
}

impl MonteCarloInputs {
    pub fn gather<T: FinancialOption>(
        option: &T,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
    ) -> MonteCarloInputs {
        let delta_t = get_duration_in_years(valuation_time, option.expiry());
        MonteCarloInputs {
            delta_t,
            underlying_price: risk_factors.underlying_price,
            underlying_volatility: risk_factors.underlying_volatility,
            annualised_historic_return: risk_factors.annualised_historic_return,
        }
    }
}

pub struct MonteCarloParams {
    pub steps: u64,
    pub repetitions: u64,
}
