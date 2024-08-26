mod absolute;
mod absolute_time;
mod float_shock;
mod relative;

pub use absolute::AbsoluteShock;
pub use absolute_time::AbsoluteTimeShock;
pub use float_shock::FloatShock;
pub use relative::RelativeShock;

pub enum ShockDirection {
    Up,
    Down,
}

pub enum ShockSize {
    AbsoluteShock(AbsoluteShock),
    RelativeShock(RelativeShock),
}

pub enum TimeShockSize {
    AbsoluteShock(AbsoluteTimeShock),
    RelativeShock(RelativeShock),
}

impl FloatShock for ShockSize {
    fn apply(&self, base: f64) -> f64 {
        match self {
            ShockSize::AbsoluteShock(shock) => shock.apply(base),
            ShockSize::RelativeShock(shock) => shock.apply(base),
        }
    }
}

impl FloatShock for TimeShockSize {
    fn apply(&self, base: f64) -> f64 {
        match self {
            TimeShockSize::AbsoluteShock(shock) => shock.apply(base),
            TimeShockSize::RelativeShock(shock) => shock.apply(base),
        }
    }
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
