use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn return_one() -> PyResult<i32> {
    return Ok(1)
}

/// A Python module implemented in Rust.
#[pymodule]
fn magent_autonomy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(return_one, m)?)?;
    Ok(())
}