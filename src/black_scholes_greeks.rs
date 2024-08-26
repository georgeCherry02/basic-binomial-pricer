use crate::black_scholes::BlackScholes;
use crate::greeks::FiniteDifferenceGreeks;
use crate::result::{make_not_implemented_error, PricerResult};

impl<T> FiniteDifferenceGreeks for T
where
    T: BlackScholes,
{
    fn delta() -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
    fn rho() -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
    fn theta() -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
    fn vega() -> PricerResult<f64> {
        Err(make_not_implemented_error())
    }
}
