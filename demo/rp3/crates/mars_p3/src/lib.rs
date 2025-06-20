// use pyo3::prelude::*;
// use ::mars::opt::grb::addtwo;

// #[pyfunction]
// fn addtwo_py(x: i32, y: i32) -> PyResult<i32> {
//     Ok(addtwo(x, y))
// }

// // #[pymodule]
// // fn grb(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
// //     m.add_function(wrap_pyfunction!(addtwo_py, m)?);
// //     Ok(())
// // }

// #[pymodule]
// fn mars(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
//     let opt = PyModule::new_bound(_py, "opt")?;
//     let grb = PyModule::new_bound(_py, "grb")?;
//     grb.add_function(wrap_pyfunction!(addtwo_py, grb.clone())?)?;
//     opt.add_submodule(&grb)?;
//     m.add_submodule(&opt)?;
//     Ok(())
// }

// pub mod sim;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use ::mars::opt::grb as grb_rs;

#[pyfunction]
fn addtwo(x: i32, y: i32) -> PyResult<i32> {
    Ok(grb_rs::addtwo(x, y))
}

// #[pymodule]
// fn mars(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
//     // Create grb
//     let grb = PyModule::new_bound(py, "grb")?;
//     grb.add_function(wrap_pyfunction!(addtwo_py, grb.clone())?)?;

//     // Create opt and add grb
//     let opt = PyModule::new_bound(py, "opt")?;
//     opt.add_submodule(&grb)?;
//     opt.setattr("grb", grb)?;

//     // Add opt to root `mars`
//     m.add_submodule(&opt)?;
//     m.setattr("opt", opt)?; // <- 🔥 THIS is the key to fix your problem

//     Ok(())
// }


// // worked
// #[pymodule]
// fn mars(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
//     // Step 1: Create nested submodules
//     let grb = PyModule::new_bound(py, "mars.opt.grb")?;
//     grb.add_function(wrap_pyfunction!(addtwo_py, grb.clone())?)?;

//     let opt = PyModule::new_bound(py, "mars.opt")?;
//     opt.add_submodule(&grb)?;
//     opt.setattr("grb", grb.clone())?;

//     // Step 2: Attach opt to the root mars module
//     m.add_submodule(&opt)?;
//     m.setattr("opt", opt.clone())?;

//     // Step 3: Register the nested modules in sys.modules (essential!)
//     let sys = py.import_bound("sys")?;
//     let modules = sys.getattr("modules")?;
//     modules.set_item("mars.opt", &opt)?;
//     modules.set_item("mars.opt.grb", &grb)?;

//     Ok(())
// }


// // worked as well
// #[pymodule]
// fn grb(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(addtwo_py, m)?)?;
//     Ok(())
// }

// #[pymodule]
// fn opt(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
//     m.add_wrapped(wrap_pymodule!(grb))?;
//     Ok(())
// }

// #[pymodule]
// fn mars(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
//     m.add_wrapped(wrap_pymodule!(opt))?;

//     // Build info - version
//     m.add("__version__", env!("CARGO_PKG_VERSION"))?;

//     // Actually not required
//     // // Register the submodules in sys.modules (essential!)
//     // let sys = py.import_bound("sys")?;
//     // let modules = sys.getattr("modules")?;
//     // modules.set_item("mars.opt", m.getattr("opt")?)?;
//     Ok(())
// }


// use pyo3::prelude::*;

mod sim;
// mod opt;

#[pymodule]
fn mars(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    let sim = PyModule::new_bound(py, "sim")?;
    sim::init_sim(py, &sim)?;
    m.add_submodule(&sim)?;

    // let opt = PyModule::new(py, "opt")?;
    // opt::init_opt(py, opt)?;
    // m.add_submodule(opt)?;

    m.add_function(wrap_pyfunction!(addtwo, m)?)?;

    Ok(())
}
