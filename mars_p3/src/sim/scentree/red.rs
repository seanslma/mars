use pyo3::prelude::*;
use mars::sim::scentree::red;

#[pyfunction]
pub fn add(x: i32, y: i32) -> PyResult<i32>  {
    let message = red::add_rd(x, y);
    Ok(message)
}
