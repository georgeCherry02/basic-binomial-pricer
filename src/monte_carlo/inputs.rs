use super::MonteCarloRiskFactors;

use crate::shock::{ApplyShock, Shock, Scenario};

use crate::utils::date::get_duration_in_years;

use chrono::{DateTime, Utc};

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
        self.risk_factors.discount_rate()
    }
    pub fn price(&self) -> f64 {
        self.risk_factors.price()
    }
    pub fn volatility(&self) -> f64 {
        self.risk_factors.volatility()
    }
    pub fn discount(&self, value: f64) -> f64 {
        value * (-self.delta_t * self.discount_rate()).exp()
    }
}

pub struct MonteCarloParams {
    pub steps: u64,
    pub repetitions: u64,
}

impl ApplyShock<MonteCarloInputs> for Shock {
    fn apply(&self, applicant: &mut MonteCarloInputs) {
        match self {
            Shock::TimeShock(shock) => shock.apply(&mut applicant.delta_t),
            _ => self.apply(&mut applicant.risk_factors),
        }
    }
}

impl ApplyShock<MonteCarloInputs> for Scenario {
    fn apply(&self, applicant: &mut MonteCarloInputs) {
        for shock in self {
            shock.apply(applicant);
        }
    }
}
