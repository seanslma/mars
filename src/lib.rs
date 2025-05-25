use pyo3::prelude::*;

/// A simple Rust function that adds two numbers
#[pyfunction]
fn add(x: i32, y: i32) -> PyResult<i32> {
    Ok(x + y)
}

/// A simple struct that will expose to Python
#[pyclass]
struct Person {
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

/// A Python module named `mars` implemented in Rust, a collection of classes/functions
#[pymodule]
fn mars(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add the function to the module
    m.add_function(wrap_pyfunction!(add, m)?)?;

    // Add the class to the module
    m.add_class::<Person>()?;

    Ok(())
}
