use pyo3::prelude::*;
use ::ders::util::math as math_rs;

#[pyfunction]
fn add(x: i32, y: i32) -> PyResult<i32> {
    Ok(math_rs::add(x, y))
}

pub fn init_mdl(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
