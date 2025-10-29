use pyo3::prelude::*;
extern crate toon as toon_lib;

#[pyfunction]
#[pyo3(signature = (data, indent=None))]
fn encode(data: String, indent: Option<usize>) -> PyResult<String> {
    // Parse JSON string
    let json_value: serde_json::Value = serde_json::from_str(&data)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid JSON: {}", e)))?;
    
    // Create options
    let options = indent.map(|i| toon_lib::EncodeOptions {
        indent: i,
        delimiter: toon_lib::Delimiter::Comma,
        length_marker: None,
    });
    
    Ok(toon_lib::encode(&json_value, options))
}

#[pymodule]
fn toon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    Ok(())
}
