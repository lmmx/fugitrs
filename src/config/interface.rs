use bevy_reflect::{Reflect, ReflectRef};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use smart_default::SmartDefault;

#[pyclass(module = "fugitrs", get_all, set_all)]
#[derive(Reflect, Serialize, Deserialize, SmartDefault, Clone, Debug, PartialEq, Eq, Hash)]
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

    fn reflecto_patronum(&self, _py: Python) {
        let instance = Config::default();
        if let ReflectRef::Struct(reflect_struct) = instance.reflect_ref() {
            // Iterate over the fields using the enumerate to get both index and field
            for (i, field) in reflect_struct.iter_fields().enumerate() {
                // Try to get the name of the field using the index
                let field_name = reflect_struct.name_at(i).unwrap_or("Unknown field");

                // Get the TypeInfo of the field
                if let Some(type_info) = field.get_represented_type_info() {
                    // Get the TypePathTable from the TypeInfo
                    let type_path_table = type_info.type_path_table();
                    println!("{}: Path: {}", field_name, type_path_table.short_path());
                } else {
                    // Handle the case where TypeInfo is not available
                    println!("{}: TypeInfo not available", field_name);
                }
            }
        } else {
            println!("The provided instance is not a Struct.");
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
