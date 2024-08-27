mod absolute;
mod absolute_time;
mod float_shock;
mod relative;
mod size;

pub use absolute::AbsoluteShock;
pub use absolute_time::AbsoluteTimeShock;
pub use float_shock::FloatShock;
pub use relative::RelativeShock;
use size::{ShockSize, TimeShockSize};

use chrono::Duration;

pub enum ShockDirection {
    Up,
    Down,
}

pub struct PriceShock {
    risk_factor_id: String,
    size: ShockSize,
}

pub struct VolatilityShock {
    risk_factor_id: String,
    size: ShockSize,
}

pub struct TimeShock {
    risk_factor_id: String,
    size: TimeShockSize,
}

pub struct InterestRateShock {
    risk_factor_id: String,
    size: ShockSize,
}

impl FloatShock for PriceShock {
    fn apply(&self, base: f64) -> f64 {
        self.size.apply(base)
    }
}
impl FloatShock for VolatilityShock {
    fn apply(&self, base: f64) -> f64 {
        self.size.apply(base)
    }
}
impl FloatShock for InterestRateShock {
    fn apply(&self, base: f64) -> f64 {
        self.size.apply(base)
    }
}
impl FloatShock for TimeShock {
    fn apply(&self, base: f64) -> f64 {
        self.size.apply(base)
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

pub const fn price_shock(risk_factor_id: String, size: ShockSize) -> Shock {
    Shock::PriceShock(PriceShock {
        risk_factor_id,
        size,
    })
}
pub const fn interest_rate_shock(risk_factor_id: String, size: ShockSize) -> Shock {
    Shock::InterestRateShock(InterestRateShock {
        risk_factor_id,
        size,
    })
}
pub const fn time_shock(risk_factor_id: String, size: TimeShockSize) -> Shock {
    Shock::TimeShock(TimeShock {
        risk_factor_id,
        size,
    })
}
pub const fn volatility_shock(risk_factor_id: String, size: ShockSize) -> Shock {
    Shock::VolatilityShock(VolatilityShock {
        risk_factor_id,
        size,
    })
}

pub type Scenario<'a> = Vec<&'a Shock>;
