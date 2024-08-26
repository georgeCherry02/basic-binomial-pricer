pub enum ShockDirection {
    Up,
    Down,
}

pub trait FloatShock {
    fn apply(&self, base: f64) -> f64;
}

pub struct AbsoluteShock {
    size: f64,
    direction: ShockDirection,
}

impl FloatShock for AbsoluteShock {
    fn apply(&self, base: f64) -> f64 {
        match self.direction {
            ShockDirection::Up => base + self.size,
            ShockDirection::Down => base - self.size,
        }
    }
}

pub struct AbsoluteTimeShock {
    size: chrono::Duration,
    direction: ShockDirection,
}

impl FloatShock for AbsoluteTimeShock {
    fn apply(&self, base: f64) -> f64 {
        const NUMBER_OF_SECONDS_IN_A_YEAR: f64 = 31536000.0;
        let shock_in_years = self.size.num_seconds() as f64 / NUMBER_OF_SECONDS_IN_A_YEAR;
        match self.direction {
            ShockDirection::Up => base + shock_in_years,
            ShockDirection::Down => base - shock_in_years,
        }
    }
}

pub enum RelativeShockType {
    Percentage,
    BasisPoint,
}

pub struct RelativeShock {
    size: f64,
    shock_type: RelativeShockType,
    direction: ShockDirection,
}

impl FloatShock for RelativeShock {
    fn apply(&self, base: f64) -> f64 {
        let amount = match self.shock_type {
            RelativeShockType::Percentage => base * self.size,
            RelativeShockType::BasisPoint => base * self.size * 0.0001,
        };
        match self.direction {
            ShockDirection::Up => base + amount,
            ShockDirection::Down => base - amount,
        }
    }
}

pub enum ShockSize {
    AbsoluteShock(AbsoluteShock),
    RelativeShock(RelativeShock),
}

pub enum TimeShockSize {
    AbsoluteTimeShock(AbsoluteShock),
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
            TimeShockSize::AbsoluteTimeShock(shock) => shock.apply(base),
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

pub type Scenario = Vec<Shock>;
