use pyo3::prelude::*;
use mars::opt::gurobi::lpsol;

#[pyfunction]
pub fn hi(name: &str) -> PyResult<String>  {
    let message = lpsol::hi(name);
    Ok(message)
}
