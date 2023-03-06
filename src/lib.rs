use obj2asterix::write_asterix;
use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict};
use pythonize::depythonize;
use serde::Deserialize;
use serde_json::{Map, Value};
use spec_parser::spec_xml::Category;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

struct Converter(BTreeMap<u8, Category>);

static CONVERTERS: Mutex<BTreeMap<u64, Arc<Converter>>> = Mutex::new(BTreeMap::new());
static LAST_ID: AtomicU64 = AtomicU64::new(0);

#[pyfunction]
fn create_converter(directory: &str, specs_to_load: &PyDict) -> PyResult<u64> {
    let new_id = LAST_ID.fetch_add(1, Ordering::Relaxed);
    let mut map: BTreeMap<u8, Category> = BTreeMap::new();
    let mut cats_map: BTreeMap<OsString, u8> = BTreeMap::new();

    for (spec_id, filename) in specs_to_load {
        cats_map.insert(filename.to_string().into(), spec_id.to_string().parse()?);
    }

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if let Some(spec_id) = cats_map.get(&entry.file_name()) {
            let xml = fs::read_to_string(entry.path())?;
            let category =
                Category::parse(&xml).map_err(|e| PyValueError::new_err(e.to_string()))?;
            map.insert(*spec_id, category);
        }
    }

    CONVERTERS
        .lock()
        .expect("poisoned mutex")
        .insert(new_id, Arc::new(Converter(map)));

    Ok(new_id)
}

#[pyfunction]
fn encode(id: u64, payload: &PyDict) -> PyResult<&PyByteArray> {
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
        let converters = CONVERTERS.lock().expect("poisoned mutex");
        let categories = converters
            .get(&id)
            .ok_or_else(|| PyKeyError::new_err("converter not created"))?
            .clone();

        let spec = categories
            .0
            .get(cat_id)
            .ok_or_else(|| PyKeyError::new_err("category not loaded"))?;
        write_asterix(&mut result, spec, &map)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(PyByteArray::new(py, &result))
    } else {
        Err(PyValueError::new_err("category ID is missing (CAT field)"))
    }
}

#[pyfunction]
fn decode(_id: u64, _payload: &PyByteArray) -> PyResult<&PyDict> {
    unimplemented!()
}

#[pymodule]
fn freeasterix(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_function(wrap_pyfunction!(create_converter, m)?)?;
    Ok(())
}
