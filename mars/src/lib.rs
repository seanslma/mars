/// Return package version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Return a greeting message
pub fn hi(name: &str) -> String {
    format!("Hello {}!", name)
}

/// Add two number
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Multiply two number
pub fn mul(x: f64, y: f64) -> f64 {
    x * y
}
