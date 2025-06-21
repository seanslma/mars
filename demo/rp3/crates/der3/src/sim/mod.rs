use pyo3::prelude::*;
use pyo3::wrap_pymodule;
pub mod mdl;

// #[pymodule]
// fn mdl(_py: Python, _m: &Bound<PyModule>) -> PyResult<()> {
//     Ok(())
// }

// #[pymodule]
// fn sim(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
//     m.add_wrapped(wrap_pymodule!(mdl))?;
//     Ok(())
// }


pub fn init_sim(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    let mdl = PyModule::new_bound(py, "mdl")?;
    mdl::init_mdl(py, &mdl)?;
    m.add_submodule(&mdl)?;

    Ok(())
}
