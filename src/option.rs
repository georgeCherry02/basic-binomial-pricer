use chrono::prelude::Utc;
use chrono::Date;

pub struct Option {
    strike: f64,
    volatility: f64,
    expiry: Date<Utc>,
}
