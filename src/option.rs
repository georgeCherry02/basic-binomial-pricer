use chrono::NaiveDate;

pub struct Option {
    strike: f64,
    volatility: f64,
    expiry: NaiveDate,
}
