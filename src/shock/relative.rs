use super::FloatShock;
use super::ShockDirection;

pub enum RelativeShockType {
    BasisPoint,
    Decimal,
    Percentage,
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
    pub const fn decimal(size: f64, direction: ShockDirection) -> RelativeShock {
        RelativeShock {
            size,
            shock_type: RelativeShockType::Decimal,
            direction,
        }
    }
}

impl FloatShock for RelativeShock {
    fn apply_float(&self, base: f64) -> f64 {
        let scaling = match self.shock_type {
            RelativeShockType::BasisPoint => 0.0001,
            RelativeShockType::Decimal => 1.0,
            RelativeShockType::Percentage => 0.01,
        };
        let amount = base * self.size * scaling;
        match self.direction {
            ShockDirection::Up => base + amount,
            ShockDirection::Down => base - amount,
        }
    }
}
