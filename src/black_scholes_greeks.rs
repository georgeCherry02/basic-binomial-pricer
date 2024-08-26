use crate::black_scholes::BlackScholes;
use crate::greeks::FiniteDifferenceGreeks;
use crate::result::{make_not_implemented_error, PricerResult};

impl<T> FiniteDifferenceGreeks for T
where
    T: BlackScholes,
{
    fn delta(&self) -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
    fn rho(&self) -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
    fn theta(&self) -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
    fn vega(&self) -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
}
