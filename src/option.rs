use crate::symbol::Symbol;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use chrono::prelude::Utc;
use chrono::DateTime;

use std::fmt;

#[pyclass(frozen)]
pub struct Call {
    symbol: Symbol,
    strike: f64,
    expiry: DateTime<Utc>,
    cost: f64,
}

pub fn get_call(symbol: Symbol, strike: f64, expiry: DateTime<Utc>, cost: f64) -> Call {
    Call {
        symbol,
        strike,
        expiry,
        cost,
    }
}

fn parse_dt(expiry_str: String) -> PyResult<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&expiry_str)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse datetime {}", e).to_string()))
        .map(|exp| exp.into())
}

#[pymethods]
impl Call {
    #[new]
    pub fn new(symbol: String, strike: f64, expiry_str: String, cost: f64) -> PyResult<Call> {
        parse_dt(expiry_str).map(|expiry| get_call(symbol.into(), strike, expiry, cost))
    }
}

#[pyclass]
pub struct Put {
    symbol: Symbol,
    strike: f64,
    expiry: DateTime<Utc>,
    cost: f64,
}

pub fn get_put(symbol: Symbol, strike: f64, expiry: DateTime<Utc>, cost: f64) -> Put {
    Put {
        symbol,
        strike,
        expiry,
        cost,
    }
}

#[pymethods]
impl Put {
    #[new]
    pub fn new(symbol: String, strike: f64, expiry_str: String, cost: f64) -> PyResult<Put> {
        parse_dt(expiry_str).map(|expiry| get_put(symbol.into(), strike, expiry, cost))
    }
}

pub trait FinancialOption {
    fn symbol(&self) -> &Symbol;
    fn strike(&self) -> f64;
    fn expiry(&self) -> DateTime<Utc>;
    fn cost(&self) -> f64;
    fn value_if_executed(&self, underlying_value: f64) -> f64;
}

impl FinancialOption for Call {
    fn symbol(&self) -> &Symbol {
        &self.symbol
    }
    fn strike(&self) -> f64 {
        self.strike
    }
    fn expiry(&self) -> DateTime<Utc> {
        self.expiry
    }
    fn cost(&self) -> f64 {
        self.cost
    }
    fn value_if_executed(&self, underlying_value: f64) -> f64 {
        underlying_value - self.strike()
    }
}

impl FinancialOption for Put {
    fn symbol(&self) -> &Symbol {
        &self.symbol
    }
    fn strike(&self) -> f64 {
        self.strike
    }
    fn expiry(&self) -> DateTime<Utc> {
        self.expiry
    }
    fn cost(&self) -> f64 {
        self.cost
    }
    fn value_if_executed(&self, underlying_value: f64) -> f64 {
        self.strike() - underlying_value
    }
}

fn local_fmt<T: FinancialOption>(t: &T, prefix: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "{}[symbol={},strike={}, expiry={}, cost={}]",
        prefix,
        t.symbol(),
        t.strike(),
        t.expiry(),
        t.cost(),
    )
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        local_fmt(self, "Call", f)
    }
}

impl fmt::Display for Put {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        local_fmt(self, "Put", f)
    }
}
