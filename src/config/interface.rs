use pyo3::prelude::*;
use smart_default::SmartDefault;

#[pyclass(module = "fugitrs", get_all, set_all)]
#[derive(SmartDefault, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Config {
    #[default = 1]
    pub param1: i32,
    pub param2: String,
    #[default = true]
    pub param3: bool,
}

#[pymethods]
impl Config {
    #[new]
    fn new(param1: Option<i32>, param2: Option<String>, param3: Option<bool>) -> Self {
        Config {
            param1: param1.unwrap_or_default(),
            param2: param2.unwrap_or_default(),
            param3: param3.unwrap_or_default(),
        }
    }
}
