pub mod opt;
pub mod sim;

use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};

// #[pyfunction]
// pub fn version() -> PyResult<String>  {
//     Ok(mars::version().to_string())
// }


/// A simple struct that will expose to Python
#[pyclass]
pub struct Person {
    name: String,
}

#[pymethods]
impl Person {
    /// A constructor: Person::new("John")
    #[new]
    fn new(name: String) -> Self {
        Person { name }
    }

    /// A method that returns a greeting message
    fn hello(&self) -> PyResult<String> {
        Ok(format!("Hello, I'm {}!", self.name))
    }
}

#[pymodule]
fn mars_opt(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(opt::gurobipy::lpsol::hi, m)?)?;
    Ok(())
}

/// A Python module named `mars` implemented in Rust, a collection of classes/functions
#[pymodule]
fn mars(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // Submodules
    m.add_wrapped(wrap_pymodule!(mars_opt))?;

    // Wrap function `hi` directly from crate `mars`
    #[pyfn(m)]
    #[pyo3(name = "hi")] // Python function name
    fn hi_py(_py: Python, name: String) -> PyResult<String> {
        // Wrap the call to mars::hi in an unsafe block as required by pyo3
        Ok(opt::gurobipy::lpsol::hi(&name)?)
    }

    // Wrap function `add` directly from crate `mars`
    #[pyfn(m)]
    #[pyo3(name = "add_rd")] // Python function name
    fn add_rd(_py: Python, x: i32, y: i32) -> PyResult<i32> {
        Ok(sim::scentree::red::add(x, y)?)
    }

    // Wrap function `mul` directly from crate `mars`
    #[pyfn(m)]
    #[pyo3(name = "add_tr")] // Python function name
    fn add_tr(_py: Python, x: i32, y: i32) -> PyResult<i32> {
        Ok(sim::scentree::tree::add(x, y)?)
    }

    // Add function in the crate itself (marked with #[pyfunction]) to module
    // m.add_function(wrap_pyfunction!(version, m)?)?;

    // Add the class in the crate itself to the module
    m.add_class::<Person>()?;

    // Version
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
