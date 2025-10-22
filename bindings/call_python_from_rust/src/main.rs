use pyo3::prelude::*;
use pyo3::types::PyModule;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let py_code = include_str!("script.py");
        let py_module = PyModule::from_code_bound(py, py_code, "script.py", "script")?;
        let py_function = py_module.getattr("call_from_rust")?;
        let result:i32 = py_function.call1((3,))?.extract()?;
        println!("Result from Python function: {}", result);
        Ok(())
    })
}
