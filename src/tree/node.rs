use chrono::prelude::Utc;
use chrono::DateTime;

#[derive(Clone, Debug, Eq, Hash)]
pub struct Position {
    pub num_ups: usize,
    pub num_downs: usize,
}

impl Position {
    pub fn get_branches(&self) -> (Position, Position) {
        (
            Position {
                num_ups: self.num_ups + 1,
                num_downs: self.num_downs,
            },
            Position {
                num_ups: self.num_ups,
                num_downs: self.num_downs + 1,
            },
        )
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.num_ups == other.num_ups && self.num_downs == other.num_downs
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub price: f64,
    pub datetime: DateTime<Utc>,
    pub pos: Position,
}
