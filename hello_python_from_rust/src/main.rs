// readme/main page is out of date?
// https://docs.rs/pyo3/latest/pyo3/#example-using-python-from-rust
// retrieved for 0.27.2

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::ffi::c_str;

fn main() -> PyResult<()> {
    println!("Hello, world!");

    Python::attach(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        
        let locals = [("os", py.import("os")?)].into_py_dict(py)?;
        let code = c_str!("os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'");
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}
