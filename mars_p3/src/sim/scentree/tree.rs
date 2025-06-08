use pyo3::prelude::*;
use mars::sim::scentree::tree;

#[pyfunction]
pub fn add(x: i32, y: i32) -> PyResult<i32>  {
    let message = tree::add_tr(x, y);
    Ok(message)
}
