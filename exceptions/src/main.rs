use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use std::fmt;

#[derive(Debug)]
struct CustomIOError;

impl std::error::Error for CustomIOError {}

impl fmt::Display for CustomIOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no!")
    }
}

impl std::convert::From<CustomIOError> for PyErr {
    fn from(err: CustomIOError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}

pub struct Connection { /* ... */}

fn bind(addr: String) -> Result<Connection, CustomIOError> {
    if &addr == "0.0.0.0"{
        Err(CustomIOError)
    } else {
        Ok(Connection{ /* ... */})
    }
}

#[pyfunction]
fn connect(s: String) -> Result<(), CustomIOError> {
    bind(s)?;
    Ok(())
}

fn main() {
    Python::attach(|py| {
        let fun = pyo3::wrap_pyfunction!(connect, py).unwrap();
        let err = fun.call1(("0.0.0.0",)).unwrap_err();
        println!("err = {:?}", err);
        //assert!(err.is_instance(py, PyOSError)); // uh, not sure how to get this syntax to work
        assert!(err.is_instance_of::<PyOSError>(py));
    });
}
