use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use chrono::prelude::Utc;
use chrono::DateTime;

use std::fmt;

#[pyclass]
pub struct Call {
    strike: f64,
    expiry: DateTime<Utc>,
}

pub fn get_call(strike: f64, expiry: DateTime<Utc>) -> Call {
    Call { strike, expiry }
}

fn parse_dt(expiry_str: String) -> PyResult<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&expiry_str)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse datetime {}", e).to_string()))
        .map(|exp| exp.into())
}

#[pymethods]
impl Call {
    #[new]
    pub fn new(strike: f64, expiry_str: String) -> PyResult<Call> {
        parse_dt(expiry_str).map(|expiry| get_call(strike, expiry))
    }
}

#[pyclass]
pub struct Put {
    strike: f64,
    expiry: DateTime<Utc>,
}

pub fn get_put(strike: f64, expiry: DateTime<Utc>) -> Put {
    Put { strike, expiry }
}

#[pymethods]
impl Put {
    #[new]
    pub fn new(strike: f64, expiry_str: String) -> PyResult<Put> {
        parse_dt(expiry_str).map(|expiry| get_put(strike, expiry))
    }
}

pub trait FinancialOption {
    fn strike(&self) -> f64;
    fn expiry(&self) -> DateTime<Utc>;
    fn value_if_executed(&self, underlying_value: f64) -> f64;
}

impl FinancialOption for Call {
    fn strike(&self) -> f64 {
        self.strike
    }
    fn expiry(&self) -> DateTime<Utc> {
        self.expiry
    }
    fn value_if_executed(&self, underlying_value: f64) -> f64 {
        underlying_value - self.strike()
    }
}

impl FinancialOption for Put {
    fn strike(&self) -> f64 {
        self.strike
    }
    fn expiry(&self) -> DateTime<Utc> {
        self.expiry
    }
    fn value_if_executed(&self, underlying_value: f64) -> f64 {
        self.strike() - underlying_value
    }
}

fn local_fmt<T: FinancialOption>(t: &T, prefix: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "{}[strike={}, expiry={}]",
        prefix,
        t.strike(),
        t.expiry(),
    )
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        local_fmt(self, "Call", f)
    }
}

impl fmt::Display for Put {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        local_fmt(self, "Call", f)
    }
}
