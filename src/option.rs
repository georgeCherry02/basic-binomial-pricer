use chrono::DateTime;

pub struct Option {
    strike: f64,
    volatility: f64,
    expiry: DateTime,
}
