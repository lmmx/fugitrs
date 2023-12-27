mod config;
mod git_diff;

use pyo3::prelude::*;

#[pymodule]
fn fugitrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(git_diff::get_git_diff, m)?)?;
    m.add_function(wrap_pyfunction!(config::print_config, m)?)?;
    Ok(())
}
