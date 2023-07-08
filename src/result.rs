use std::{error, fmt};

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
