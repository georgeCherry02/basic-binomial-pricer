use super::FloatShock;
use super::ShockDirection;

pub struct AbsoluteShock {
    size: f64,
    direction: ShockDirection,
}

impl AbsoluteShock {
    pub const fn new(size: f64, direction: ShockDirection) -> AbsoluteShock {
        AbsoluteShock { size, direction }
    }
}

impl FloatShock for AbsoluteShock {
    fn apply(&self, base: f64) -> f64 {
        match self.direction {
            ShockDirection::Up => base + self.size,
            ShockDirection::Down => base - self.size,
        }
    }
}
