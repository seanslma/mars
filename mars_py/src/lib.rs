use pyo3::prelude::*;
use mars_p3; // Import the 'mars_p3' crate

#[pyfunction]
pub fn version() -> String {
    mars::version().to_string()
}


/// A Python module named `mars` implemented in Rust, a collection of classes/functions
#[pymodule]
fn mars_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Wrap function `hi` directly from crate `mars`
    #[pyfn(m)]
    #[pyo3(name = "hi")] // Python function name
    fn hi_py(_py: Python, name: String) -> PyResult<String> {
        // Wrap the call to mars::hi in an unsafe block as required by pyo3
        Ok(mars::hi(&name))
    }

    // Wrap function `add` directly from crate `mars`
    #[pyfn(m)]
    #[pyo3(name = "add")] // Python function name
    fn add_py(_py: Python, x: i32, y: i32) -> PyResult<i32> {
        Ok(mars::add(x, y))
    }

    // Wrap function `mul` directly from crate `mars`
    #[pyfn(m)]
    #[pyo3(name = "mul")] // Python function name
    fn mul_py(_py: Python, x: f64, y: f64) -> PyResult<f64> {
        Ok(mars::mul(x, y))
    }

    // Add function in the crate itself (marked with #[pyfunction]) to module
    m.add_function(wrap_pyfunction!(version, m)?)?;

    // Add the class in the crate itself to the module
    m.add_class::<Person>()?;

    // Version
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
