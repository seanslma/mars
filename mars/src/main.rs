pub mod opt {
    pub mod gurobi {
        pub mod lpsol;
    }
}

use std::time::Instant;
use std::path::PathBuf;
use core::solver::lpsol::{
    process_lp_file,
    // process_sol_file,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/test";
    let lpi = "model.lp";
    let path_buf = PathBuf::from(path).join(lpi);
    let input_file = path_buf.as_path();

    let start_time = Instant::now();
    process_lp_file(input_file)?;
    let elapsed = start_time.elapsed();
    println!("LP processing completed in {:.3?}", elapsed);

    Ok(())
}

// fn main() {
//     let name = "John";
//     println!("Hello, world! This is {}.", name);
// }
