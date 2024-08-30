use super::common::{gaussian, get_d1_and_d2};
use super::types::{BlackScholesInputs, BlackScholesRiskFactors};

use crate::option::{Call, FinancialOption, Put};
use crate::result::{PricerError, PricerResult};
use crate::risk_factors::RiskFactors;
use crate::shock::{ApplyShock, Scenario};
use crate::symbol::Symbol;

use crate::risk_factors::discount::DiscountFactor;
use crate::risk_factors::dividend::{AnnualisedDividendRate, Dividend};
use crate::risk_factors::price::{Price, PriceTick};
use crate::risk_factors::volatility::{ImpliedVolatility, Volatility};

use chrono::{DateTime, Utc};

use statrs::distribution::ContinuousCDF;

fn insensitive_risk_factor_err(risk_factor: &Symbol, symbol: &Symbol) -> PricerError {
    PricerError::new(
        format!(
            "Provided risk factor with symbol {}, when option is sensitive to symbol {}",
            risk_factor, symbol
        ),
        1,
    )
}
fn check_symbols(risk_factor: &Symbol, symbol: &Symbol) -> PricerResult<()> {
    if risk_factor != symbol {
        Err(insensitive_risk_factor_err(risk_factor, symbol))
    } else {
        Ok(())
    }
}

pub trait BlackScholes: FinancialOption {
    fn is_sensitive_to_risk_factors(
        &self,
        risk_factors: &BlackScholesRiskFactors,
    ) -> PricerResult<()> {
        check_symbols(risk_factors.price_risk_factor(), self.symbol())?;
        check_symbols(risk_factors.volatility_risk_factor(), self.symbol())?;
        check_symbols(risk_factors.dividend_risk_factor(), self.symbol())?;
        Ok(())
    }
    fn get_black_scholes_risk_factors(
        &self,
        price: f64,
        volatility: f64,
        dividend_rate: f64,
        discount_factor: DiscountFactor,
    ) -> RiskFactors {
        RiskFactors {
            price_sensitivities: vec![Price::PriceTick(PriceTick::new(
                self.symbol().clone(),
                price,
            ))],
            volatility_sensitivities: vec![Volatility::ImpliedVolatility(ImpliedVolatility::new(
                self.symbol().clone(),
                volatility,
            ))],
            discount_factors: vec![discount_factor],
            dividend_sensitivities: vec![Dividend::AnnualisedRate(AnnualisedDividendRate::new(
                self.symbol().clone(),
                dividend_rate,
            ))],
        }
    }
    fn value_black_scholes_impl(&self, inputs: BlackScholesInputs) -> PricerResult<f64>;
    fn value_black_scholes(
        &self,
        valuation_time: DateTime<Utc>,
        risk_factors: RiskFactors,
        shock_scenarios: Scenario,
    ) -> PricerResult<f64> {
        let check_sensitivity_to_risk_factors = |risk_factors| {
            self.is_sensitive_to_risk_factors(&risk_factors)?;
            Ok(risk_factors)
        };
        let gather_model_inputs =
            |risk_factors| BlackScholesInputs::gather(self.expiry(), valuation_time, risk_factors);
        let shock_inputs = |mut inputs| {
            shock_scenarios.apply(&mut inputs);
            inputs
        };
        risk_factors
            .try_into()
            .and_then(check_sensitivity_to_risk_factors)
            .map(gather_model_inputs)
            .map(shock_inputs)
            .and_then(|input| self.value_black_scholes_impl(input))
    }
}

impl BlackScholes for Call {
    fn value_black_scholes_impl(&self, inputs: BlackScholesInputs) -> PricerResult<f64> {
        let (d1, d2) = get_d1_and_d2(self.strike(), &inputs);
        gaussian()
            .map(|gaussian| {
                inputs.dividend_adjusted_price() * gaussian.cdf(d1)
                    - self.strike() * inputs.risk_free_adjustment() * gaussian.cdf(d2)
            })
            .map(|valuation| valuation - self.cost())
    }
}

impl BlackScholes for Put {
    fn value_black_scholes_impl(&self, inputs: BlackScholesInputs) -> PricerResult<f64> {
        let (d1, d2) = get_d1_and_d2(self.strike(), &inputs);
        gaussian()
            .map(|gaussian| {
                self.strike() * inputs.risk_free_adjustment() * gaussian.cdf(-d2)
                    - inputs.dividend_adjusted_price() * gaussian.cdf(-d1)
            })
            .map(|valuation| valuation - self.cost())
    }
}
