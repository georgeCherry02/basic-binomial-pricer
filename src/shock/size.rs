use super::FloatShock;
use super::{AbsoluteShock, AbsoluteTimeShock, RelativeShock};

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
