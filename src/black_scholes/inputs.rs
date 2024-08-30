use super::risk_factors::BlackScholesRiskFactors;

use crate::shock::{ApplyShock, Scenario, Shock};

use crate::utils::date::get_duration_in_years;

use chrono::{DateTime, Utc};

pub struct BlackScholesInputs {
    pub delta_t: f64,
    risk_factors: BlackScholesRiskFactors,
}

impl BlackScholesInputs {
    pub fn gather(
        expiry: DateTime<Utc>,
        valuation_time: DateTime<Utc>,
        risk_factors: BlackScholesRiskFactors,
    ) -> BlackScholesInputs {
        let delta_t = get_duration_in_years(valuation_time, expiry);
        BlackScholesInputs {
            delta_t,
            risk_factors,
        }
    }
    pub fn discount_rate(&self) -> f64 {
        self.risk_factors.discount_rate()
    }
    pub fn annualised_dividend_rate(&self) -> f64 {
        self.risk_factors.annualised_dividend_rate()
    }
    pub fn price(&self) -> f64 {
        self.risk_factors.price()
    }
    pub fn volatility(&self) -> f64 {
        self.risk_factors.volatility()
    }
    pub fn risk_free_adjustment(&self) -> f64 {
        self.risk_factors.discount_factor(self.delta_t)
    }
    pub fn volatility_for_delta_t(&self) -> f64 {
        self.risk_factors.volatility_for_delta_t(self.delta_t)
    }

    pub fn dividend_adjustment(&self) -> f64 {
        (-self.annualised_dividend_rate() * self.delta_t).exp()
    }
    pub fn dividend_adjusted_price(&self) -> f64 {
        self.price() * self.dividend_adjustment()
    }
}

impl ApplyShock<BlackScholesInputs> for Shock {
    fn apply(&self, applicant: &mut BlackScholesInputs) {
        match self {
            Shock::TimeShock(shock) => shock.apply(&mut applicant.delta_t),
            _ => self.apply(&mut applicant.risk_factors),
        }
    }
}

impl ApplyShock<BlackScholesInputs> for Scenario {
    fn apply(&self, applicant: &mut BlackScholesInputs) {
        for shock in self {
            shock.apply(applicant);
        }
    }
}
