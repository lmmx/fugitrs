use pyo3::prelude::*;
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug)]
pub struct Config {
    #[default = 1]
    param1: i32,
    param2: String,
    #[default = true]
    param3: bool,
}

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
