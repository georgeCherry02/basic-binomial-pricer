pub struct MonteCarloInputs {
    pub delta_t: f64,
    pub underlying_price: f64,
    pub underlying_volatility: f64,
    pub annualised_historic_return: f64,
}

impl MonteCarloInputs {
    pub fn volatility_for_delta_t(&self) -> f64 {
        self.underlying_volatility * self.delta_t.sqrt()
    }
}

pub struct MonteCarloParams {
    pub steps: u64,
    pub repetitions: u64,
}
