mod config;
mod git;

use pyo3::prelude::*;

#[pymodule]
fn fugitrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(git::get_git_diff, m)?)?;
    m.add_function(wrap_pyfunction!(config::print_config, m)?)?;
    Ok(())
}
