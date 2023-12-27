use super::Config;
use pyo3::prelude::*; // Import from the parent module (config)

pub fn _print_config(config: Config) {
    println!(
        "1: {:?}, 2: {:?}, 3: {:?}",
        config.param1, config.param2, config.param3
    );
}

#[pyfunction]
pub fn print_config() -> PyResult<()> {
    _print_config(Config::default());
    Ok(())
}
