use crate::result::PricerResult;

pub trait FiniteDifferenceGreeks {
    fn delta() -> PricerResult<f64>;
    fn rho() -> PricerResult<f64>;
    fn vega() -> PricerResult<f64>;
    fn theta() -> PricerResult<f64>;
}
