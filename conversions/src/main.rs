/*
use pyo3::prelude::*;
use pyo3::ffi::c_str;
#[derive(FromPyObject)]
struct RustyStruct {
    my_string: String,
}


fn main() -> PyResult<()> {
    Python::attach(|py| -> PyResult<()> {
        let module = PyModule::from_code(
            py,
            c_str!(
r#"
class Foo:
    def __init__(self):
        self.my_string = 'test'
"#
            ),
            c_str!(""),
            c_str!(""),
        )?;
        
        let class = module.getattr("Foo")?;
        let instance = class.call0()?;
        let rustystruct: RustyStruct = instance.extract()?;
        assert_eq!(rustystruct.my_string, "test");
        println!("rustystruct.my_string = {}", rustystruct.my_string);
        //println!("rustystruct.__class__.__name__ = {}", rustystruct.__class__.__name__); // fails
        Ok(())
    })
}
*/


use pyo3::prelude::*;
use pyo3::types::PyDict;

#[derive(FromPyObject)]
struct RustyStruct {
    #[pyo3(item("blah"))]
    my_string: String,

    #[pyo3(item)]
    my_int: i32,
}

fn main() -> PyResult<()> {
    Python::attach(|py| -> PyResult<()> {
        let dict = PyDict::new(py);
        dict.set_item("my_int", 32)?;
        dict.set_item("blah", "test")?;
        let rustystruct: RustyStruct = dict.extract()?;
        
        assert_eq!(rustystruct.my_string, "test");
        println!("rustystruct.my_string = {}", rustystruct.my_string);
        println!("rustystruct.my_int = {}", rustystruct.my_int);
        Ok(())
    })
}

