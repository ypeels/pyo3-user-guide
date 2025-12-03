use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple, IntoPyDict};
use pyo3::ffi::c_str;


/// https://pyo3.rs/v0.27.2/python-from-rust/function-calls
fn main() -> PyResult<()> {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    let key1 = "key1";
    let val1 = 1;
    let key2 = "key2";
    let val2 = 2;

    Python::attach(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            c_str!(r#"def example(*args, **kwargs):        
                        tokens = []
                        if args != ():
                            print('called with args', args)
                            tokens.append('args')
                        if kwargs != {}:
                            print('called with kwargs', kwargs)
                            tokens.append('kwargs')
                        if args == () and kwargs == {}:
                            print('called with no arguments')
                        return ', '.join(tokens)

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
        fun.call1(py, &args)?;

        // call object with PyDict
        let kwargs = [(key1, val1)].into_py_dict(py)?;
        fun.call(py, (), Some(&kwargs))?;

        // pass arguments as Vec
        let kwargs = vec![(key1, val1), (key2, val2)];
        fun.call(py, (), Some(&kwargs.into_py_dict(py)?))?;

        // pass arguments as HashMap
        let mut kwargs = HashMap::<&str, i32>::new();
        kwargs.insert(key1, 1);
        fun.call(py, (), Some(&kwargs.into_py_dict(py)?))?;

        // pass arguments of different types as PyDict
        let kwargs = PyDict::new(py);
        kwargs.set_item(key1, val1)?;
        kwargs.set_item(key2, "string")?;

        let kwargs = PyDict::new(py);
        kwargs.set_item("hello", "world");
        let result = fun.call(py, &args, Some(&kwargs))?;
        println!("call() result {:?}, unpacked {:?}", result, result.extract::<&str>(py)?);


        Ok(())
    })
}
