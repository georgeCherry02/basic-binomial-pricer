use crate::result::PricerResult;

pub trait FiniteDifferenceGreeks {
    fn delta(&self) -> PricerResult<f64>;
    fn rho(&self) -> PricerResult<f64>;
    fn vega(&self) -> PricerResult<f64>;
    fn theta(&self) -> PricerResult<f64>;
}
