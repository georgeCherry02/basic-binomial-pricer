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

fn apply_shock_size(base: f64, shock_size: &ShockSize) -> f64 {
    match shock_size {
        ShockSize::AbsoluteShock(shock) => shock.apply(base),
        ShockSize::RelativeShock(shock) => shock.apply(base),
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
    size: ShockSize,
}

pub struct InterestRateShock {
    risk_factor_id: String,
    size: ShockSize,
}

impl FloatShock for PriceShock {
    fn apply(&self, base: f64) -> f64 {
        apply_shock_size(base, &self.size)
    }
}
impl FloatShock for VolatilityShock {
    fn apply(&self, base: f64) -> f64 {
        apply_shock_size(base, &self.size)
    }
}
impl FloatShock for InterestRateShock {
    fn apply(&self, base: f64) -> f64 {
        apply_shock_size(base, &self.size)
    }
}
impl FloatShock for TimeShock {
    fn apply(&self, base: f64) -> f64 {
        apply_shock_size(base, &self.size)
    }
}

pub enum Shock {
    PriceShock(PriceShock),
    VolatilityShock(VolatilityShock),
    TimeShock(TimeShock),
    InterestRateShock(InterestRateShock),
}

pub type Scenario = Vec<Shock>;
