use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use smart_default::SmartDefault;

#[pyclass(module = "fugitrs", get_all, set_all)]
#[derive(Serialize, Deserialize, SmartDefault, Clone, Debug, PartialEq, Eq, Hash)]
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

    fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let serialized = to_value(self).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to serialize: {}", e))
        })?;
        convert_to_pyobject(py, &serialized)
    }
}

// Helper function to convert serde_json::Value to PyObject
fn convert_to_pyobject(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = num.as_f64() {
                Ok(f.into_py(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Invalid number",
                ))
            }
        }
        Value::String(s) => Ok(s.clone().into_py(py)),
        Value::Array(arr) => {
            let py_list = PyList::empty(py);
            for item in arr {
                py_list.append(convert_to_pyobject(py, item)?)?;
            }
            Ok(py_list.to_object(py))
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (k, v) in obj {
                py_dict.set_item(k, convert_to_pyobject(py, v)?)?;
            }
            Ok(py_dict.to_object(py))
        }
    }
}
