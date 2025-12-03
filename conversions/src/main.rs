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
                "class Foo:
                def __init__(self):
                    self.my_string = 'test'"
            ),
            c_str!(""),
            c_str!(""),
        )?;

        let class = module.getattr("Foo")?;
        let instance = class.call0()?;
        let rustystruct: RustyStruct = instance.extract()?;
        assert_eq!(rustystruct.my_string, "test");
        println!("rustystruct.my_string = {}", rustystruct.my_string);
        Ok(())
    })
}