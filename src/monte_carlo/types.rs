pub struct MonteCarloInputs {
    pub delta_t: f64,
    pub underlying_price: f64,
    pub underlying_volatility: f64,
    pub annualised_historic_return: f64,
}

pub struct MonteCarloParams {
    pub steps: u64,
    pub repetitions: u64,
}
