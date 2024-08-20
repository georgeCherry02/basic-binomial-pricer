use std::{error, fmt};

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::PyErr;

#[derive(Debug)]
pub struct PricerError {
    pub message: String,
    pub code: u64,
}

impl fmt::Display for PricerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code: {}, Message: {}", self.code, self.message)
    }
}

impl error::Error for PricerError {}

pub type PricerResult<T> = Result<T, PricerError>;

pub fn make_not_implemented_error() -> PricerError {
    PricerError {
        message: String::from("Behaviour not implemented yet"),
        code: 999,
    }
}

impl std::convert::From<PricerError> for PyErr {
    fn from(value: PricerError) -> Self {
        PyRuntimeError::new_err(value.message)
    }
}
