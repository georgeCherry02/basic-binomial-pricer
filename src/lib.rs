mod option;

use option::{Call, Put};
use pyo3::prelude::*;

#[pyfunction]
pub fn price_binomial() -> PyResult<f64> {
    Ok(5.0)
}

#[pymodule]
fn pricer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(price_binomial, m)?)?;
    m.add_class::<Put>()?;
    m.add_class::<Call>()?;
    Ok(())
}
