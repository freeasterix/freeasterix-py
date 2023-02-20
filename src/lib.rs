use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict};
use pythonize::depythonize;
use serde_json::Value;
use std::collections::BTreeMap;

#[pyfunction]
fn encode(payload: &PyDict) -> PyResult<&PyByteArray> {
    let py = payload.py();
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    for (k, v) in payload {
        let key = k.to_string();
        let value: Value = depythonize(v)?;
        map.insert(key, value);
    }
    dbg!(map);
    let result = vec![1u8, 2, 3];
    Ok(PyByteArray::new(py, &result))
}

#[pymodule]
fn freeasterix(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?).unwrap();
    Ok(())
}
