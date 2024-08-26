use super::FloatShock;
use super::ShockDirection;

pub enum RelativeShockType {
    Percentage,
    BasisPoint,
}

pub struct RelativeShock {
    size: f64,
    shock_type: RelativeShockType,
    direction: ShockDirection,
}

impl RelativeShock {
    pub const fn percentage(size: f64, direction: ShockDirection) -> RelativeShock {
        RelativeShock {
            size,
            shock_type: RelativeShockType::Percentage,
            direction,
        }
    }
    pub const fn basis_point(size: f64, direction: ShockDirection) -> RelativeShock {
        RelativeShock {
            size,
            shock_type: RelativeShockType::BasisPoint,
            direction,
        }
    }
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
