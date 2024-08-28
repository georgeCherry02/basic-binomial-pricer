pub mod discount;
pub mod dividend;
pub mod price;
pub mod volatility;

#[derive(Clone)]
pub struct RiskFactors {
    pub underlying_price: f64,
    pub underlying_volatility: f64,
    pub risk_free_rate: f64,
    pub annualised_dividend_rate: f64,
    pub annualised_historic_return: f64,
}

// Make risk factors optional
// Add specialised constructors for different pricing methodologies
// Leverage gather for input checking

impl RiskFactors {
    pub fn new(
        underlying_price: f64,
        underlying_volatility: f64,
        risk_free_rate: f64,
        annualised_dividend_rate: f64,
        annualised_historic_return: f64,
    ) -> RiskFactors {
        RiskFactors {
            underlying_price,
            underlying_volatility,
            risk_free_rate,
            annualised_dividend_rate,
            annualised_historic_return,
        }
    }
}
