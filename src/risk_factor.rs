#[derive(Clone)]
pub struct RiskFactors {
    pub underlying_price: f64,
    pub underlying_volatility: f64,
    pub risk_free_rate: f64,
}

impl RiskFactors {
    pub fn new(
        underlying_price: f64,
        underlying_volatility: f64,
        risk_free_rate: f64,
    ) -> RiskFactors {
        RiskFactors {
            underlying_price,
            underlying_volatility,
            risk_free_rate,
        }
    }
}
