use pyo3::prelude::*;
use pyo3::types::PyDict;
use pythonize::depythonize;
use serde_json::Value;
use std::collections::BTreeMap;

#[pyfunction]
fn encode(payload: &PyDict) -> PyResult<Vec<u8>> {
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    for (k, v) in payload {
        let key = k.to_string();
        let value: Value = depythonize(v)?;
        map.insert(key, value);
    }
    dbg!(map);
    Ok(vec![])
}

#[pymodule]
fn freeasterix(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?).unwrap();
    Ok(())
}
