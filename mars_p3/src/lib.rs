use pyo3::prelude::*;

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
