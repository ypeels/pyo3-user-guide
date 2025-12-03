#![allow(dead_code)]
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::exceptions::PyException;

#[pyclass(extends=PyException)]
struct CustomError {
    #[pyo3(get)]
    url: String,

    #[pyo3(get)]
    message: String,
}

#[pymethods]
impl CustomError {
    #[new]
    fn new(url: String, message: String) -> Self {
        Self { url, message }
    }
}

/// https://pyo3.rs/v0.27.2/exception.html#creating-more-complex-exceptions
fn main() -> PyResult<()> {
    Python::attach(|py| {
        let ctx = [("CustomError", py.get_type::<CustomError>())].into_py_dict(py)?;
        pyo3::py_run!(
            py,
            *ctx,
            "assert str(CustomError) == \"<class 'builtins.CustomError'>\", repr(CustomError)"
        );
        pyo3::py_run!(py, *ctx, "assert CustomError('https://example.com', 'something went bad').args == ('https://example.com', 'something went bad')");
        pyo3::py_run!(py, *ctx, "assert CustomError('https://example.com', 'something went bad').url == 'https://example.com'");

        pyo3::py_run!(py, *ctx, "print(CustomError)");


        Ok(())
    })
}
