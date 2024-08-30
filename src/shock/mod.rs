mod absolute;
mod absolute_time;
mod float_shock;
mod relative;
mod size;

use float_shock::FloatShock;
use size::{ShockSize, TimeShockSize};

pub use absolute::AbsoluteShock;
pub use absolute_time::AbsoluteTimeShock;
pub use relative::RelativeShock;

use crate::symbol::Symbol;

use chrono::Duration;

pub enum ShockDirection {
    Up,
    Down,
}

pub trait ApplyShock<T> {
    fn apply(&self, applicant: &mut T);
}

pub struct PriceShock {
    risk_factor_id: Symbol,
    size: ShockSize,
}

impl PriceShock {
    pub fn risk_factor(&self) -> &Symbol {
        &self.risk_factor_id
    }
}

pub struct VolatilityShock {
    risk_factor_id: Symbol,
    size: ShockSize,
}

pub struct TimeShock {
    size: TimeShockSize,
}

pub struct InterestRateShock {
    risk_factor_id: Symbol,
    size: ShockSize,
}

impl FloatShock for PriceShock {
    fn apply_float(&self, base: f64) -> f64 {
        self.size.apply_float(base)
    }
}
impl FloatShock for VolatilityShock {
    fn apply_float(&self, base: f64) -> f64 {
        self.size.apply_float(base)
    }
}
impl FloatShock for InterestRateShock {
    fn apply_float(&self, base: f64) -> f64 {
        self.size.apply_float(base)
    }
}
impl FloatShock for TimeShock {
    fn apply_float(&self, base: f64) -> f64 {
        self.size.apply_float(base)
    }
}

impl<T> ApplyShock<f64> for T
where
    T: FloatShock,
{
    fn apply(&self, applicant: &mut f64) {
        *applicant = self.apply_float(*applicant)
    }
}

pub enum Shock {
    PriceShock(PriceShock),
    VolatilityShock(VolatilityShock),
    TimeShock(TimeShock),
    InterestRateShock(InterestRateShock),
}

pub const fn absolute_shock(size: f64, direction: ShockDirection) -> ShockSize {
    ShockSize::AbsoluteShock(AbsoluteShock::new(size, direction))
}
pub const fn absolute_time_shock(size: Duration, direction: ShockDirection) -> TimeShockSize {
    TimeShockSize::AbsoluteShock(AbsoluteTimeShock::new(size, direction))
}
pub const fn relative_percentage_shock(
    percentage_points: f64,
    direction: ShockDirection,
) -> ShockSize {
    ShockSize::RelativeShock(RelativeShock::percentage(percentage_points, direction))
}
pub const fn relative_basis_point_shock(basis_points: u64, direction: ShockDirection) -> ShockSize {
    ShockSize::RelativeShock(RelativeShock::basis_point(basis_points as f64, direction))
}

pub const fn price_shock(risk_factor_id: Symbol, size: ShockSize) -> Shock {
    Shock::PriceShock(PriceShock {
        risk_factor_id,
        size,
    })
}
pub const fn interest_rate_shock(risk_factor_id: Symbol, size: ShockSize) -> Shock {
    Shock::InterestRateShock(InterestRateShock {
        risk_factor_id,
        size,
    })
}
pub const fn time_shock(size: TimeShockSize) -> Shock {
    Shock::TimeShock(TimeShock { size })
}
pub const fn volatility_shock(risk_factor_id: Symbol, size: ShockSize) -> Shock {
    Shock::VolatilityShock(VolatilityShock {
        risk_factor_id,
        size,
    })
}

pub type Scenario = Vec<Shock>;
