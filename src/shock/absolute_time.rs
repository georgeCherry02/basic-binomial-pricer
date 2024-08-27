use super::FloatShock;
use super::ShockDirection;

use chrono::Duration;

pub struct AbsoluteTimeShock {
    size: Duration,
    direction: ShockDirection,
}

impl AbsoluteTimeShock {
    pub const fn new(size: chrono::Duration, direction: ShockDirection) -> AbsoluteTimeShock {
        AbsoluteTimeShock { size, direction }
    }
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
