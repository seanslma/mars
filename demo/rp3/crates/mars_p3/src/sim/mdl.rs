use pyo3::prelude::*;
use ::mars::opt::grb as grb_rs;

#[pyfunction]
fn addtwo(x: i32, y: i32) -> PyResult<i32> {
    Ok(grb_rs::addtwo(x, y))
}

pub fn init_mdl(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(addtwo, m)?)?;
    Ok(())
}
