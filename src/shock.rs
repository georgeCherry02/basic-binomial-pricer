pub enum ShockDirection {
    Up,
    Down,
}

pub struct AbsoluteShock {
    size: f64,
    direction: ShockDirection,
}

pub enum RelativeShockType {
    Percentage,
    BasisPoint,
}

pub struct RelativeShock {
    size: f64,
    shock_type: RelativeShockType,
}

pub enum ShockSize {
    AbsoluteShock(AbsoluteShock),
    RelativeShock(RelativeShock),
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

pub enum Shock {
    PriceShock(PriceShock),
    VolatilityShock(VolatilityShock),
    TimeShock(TimeShock),
    InterestRateShock(InterestRateShock),
}

pub type Scenario = Vec<Shock>;
