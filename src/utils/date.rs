use chrono::DateTime;
use chrono::prelude::Utc;

pub fn get_duration_in_years(t1: DateTime<Utc>, t2: DateTime<Utc>) -> f64 {
    let diff: chrono::Duration = t2 - t1;
    let diff_in_secs: i64 = diff.num_seconds();
    const NUMBER_OF_SECONDS_IN_A_YEAR: f64 = 31536000.0;
    diff_in_secs as f64 / NUMBER_OF_SECONDS_IN_A_YEAR
}

pub fn get_datetime_range(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    num_steps: i32,
) -> Vec<DateTime<Utc>> {
    let dur = end - start;
    let diff = dur / num_steps;
    let mut date_range = Vec::new();
    for i in 0..num_steps + 1 {
        date_range.push(start + (diff * i));
    }
    date_range
}
