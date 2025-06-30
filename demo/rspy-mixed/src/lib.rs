pub mod util;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn add(x: i32, y: i32) -> PyResult<i32>  {
    let z = util::add(x, y);
    Ok(z)
}

// /// A Python module implemented in Rust.
// #[pymodule]
// fn util(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(add, m)?)?;
//     Ok(())
// }


/// A Python module implemented in Rust.
#[pymodule]
fn rspy_mixed(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_wrapped(wrap_pymodule!(util))?;

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
