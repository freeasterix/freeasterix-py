use serde_json::{Map, Value};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyByteArray, PyDict, PyString, PyCapsule};
use pythonize::{pythonize, depythonize};
use std::collections::BTreeMap;
use std::ffi::CString;
use spec_parser::spec_xml::Category;
use obj2asterix::{read_asterix, write_asterix};

struct Converter {
    mapping: BTreeMap<u8, Category>,
}

fn map_py_err<E: std::error::Error>(error: E) -> PyErr {
    PyValueError::new_err(error.to_string())
}

#[pyfunction]
fn create_converter(directory: &PyString) -> PyResult<&PyCapsule> {
    let py = directory.py();
    let directory = directory.to_str()?;
    let mut converter = Converter { mapping: Default::default() };
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let spec_src = std::fs::read_to_string(entry.path())?;
        let category = Category::parse(&spec_src).map_err(map_py_err)?;
        converter.mapping.insert(category.id, category);
    }
    let name = CString::new("AxConveter").unwrap();
    let capsule = PyCapsule::new(py, converter, Some(name))?;
    Ok(capsule)
}

#[pyfunction]
fn encode<'a>(conveter: &'a PyCapsule, payload: &'a PyDict) -> PyResult<&'a PyByteArray> {
    let py = payload.py();
    let map: Map<String, Value> = depythonize(payload)?;
    let category: u8 = map.get("CAT")
        .ok_or_else(|| PyValueError::new_err("Dict must contain CAT with category"))?
        .as_u64()
        .ok_or_else(|| PyValueError::new_err("Category must be a number"))?
        .try_into()?;
    let conveter = unsafe { conveter.reference::<Converter>() };
    let spec = conveter.mapping.get(&category)
        .ok_or_else(|| PyValueError::new_err(format!("unknown category {category}")))?;
    let mut buf = Vec::new();
    write_asterix(&mut buf, &spec, &map).map_err(map_py_err)?;
    Ok(PyByteArray::new(py, &buf))
}

#[pyfunction]
fn decode<'a>(conveter: &'a PyCapsule, data: &'a PyByteArray) -> PyResult<PyObject> {
    let py = conveter.py();
    let mut reader = unsafe { data.as_bytes() };
    let category = *reader.get(0)
        .ok_or_else(|| PyValueError::new_err("ByteArray is too short"))?;
    let conveter = unsafe { conveter.reference::<Converter>() };
    let spec = conveter.mapping.get(&category)
        .ok_or_else(|| PyValueError::new_err(format!("unknown category {category}")))?;
    let value = read_asterix(&mut reader, &spec).map_err(map_py_err)?;
    let pythonic = pythonize(py, &Value::Object(value))?;
    Ok(pythonic)
}

#[pymodule]
fn freeasterix(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?).unwrap();
    m.add_function(wrap_pyfunction!(decode, m)?).unwrap();
    m.add_function(wrap_pyfunction!(create_converter, m)?).unwrap();
    Ok(())
}
