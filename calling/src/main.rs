use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::ffi::c_str;


fn main() -> PyResult<()> {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    Python::attach(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(r#"def example(*args, **kwargs):                
                        if args != ():
                            print('called with args', args)
                        if kwargs != {}:
                            print('called with kwargs', kwargs)
                        if args == () and kwargs == {}:
                            print('called with no arguments')
            "#),
            c_str!("example.py"),
            c_str!(""),
        )?
        .getattr("example")?
        .into();

        // call object without any arguments
        fun.call0(py)?;

        // pass object with Rust tuple of positional arguments
        let args = (arg1, arg2, arg3);
        fun.call1(py, args)?;

        // call object with Python tuple of positional arguments
        let args = PyTuple::new(py, &[arg1, arg2, arg3])?;
        fun.call1(py, args)?;
        Ok(())
    })
}