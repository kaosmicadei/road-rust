use pyo3::prelude::*;

fn main() -> PyResult<()> {

  Python::with_gil(|py| {
    let sys = py.import_bound("sys")?;
    sys.getattr("path")?.call_method1("append", (".",))?;
    // Append the current directory to sys.path to ensure Python can find local
    // modules. This is not a good solution but PyO3 is having issues with
    // virtual environments.

    let py_module = PyModule::import_bound(py, "pymodule")?;
    // Import the Python module `pymodule`.

    let py_function = py_module.getattr("double")?;
    // Get the Python function `double` from the module `pymodule`.


    let result:i32 = py_function.call1((3,))?.extract()?;
    // Call the Python function with an argument and extract the result.


    println!("Result from Python function: {}", result);
    Ok(())
  })
}
