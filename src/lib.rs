use obj2asterix::write_asterix;
use once_cell::sync::Lazy;
use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict};
use pythonize::depythonize;
use serde::Deserialize;
use serde_json::{Map, Value};
use spec_parser::spec_xml::Category;
use std::collections::BTreeMap;
use std::fs;
use std::sync::RwLock;

static CATEGORIES: Lazy<RwLock<BTreeMap<u8, Category>>> = Lazy::new(<_>::default);

#[pyfunction]
fn load_category_xml(path: &str) -> PyResult<()> {
    let xml = fs::read_to_string(path)?;
    let category = Category::parse(&xml).map_err(|e| PyValueError::new_err(e.to_string()))?;
    CATEGORIES.write().unwrap().insert(category.id, category);
    Ok(())
}

#[pyfunction]
fn encode(payload: &PyDict) -> PyResult<&PyByteArray> {
    let py = payload.py();
    let mut map: Map<String, Value> = <_>::default();
    let mut category_id: Option<u8> = None;
    for (k, v) in payload {
        let key = k.to_string();
        let value: Value = depythonize(v)?;
        if key == "CAT" {
            category_id.replace(
                u8::deserialize(&value).map_err(|e| PyValueError::new_err(e.to_string()))?,
            );
        }
        map.insert(key, value);
    }
    let mut result = Vec::new();
    if let Some(ref cat_id) = category_id {
        let categories = CATEGORIES.read().unwrap();
        let spec = categories
            .get(cat_id)
            .ok_or_else(|| PyKeyError::new_err("category not loaded"))?;
        write_asterix(&mut result, spec, &map)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(PyByteArray::new(py, &result))
    } else {
        Err(PyValueError::new_err("category ID is missing (CAT field)"))
    }
}

#[pymodule]
fn freeasterix(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_category_xml, m)?)?;
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    Ok(())
}
