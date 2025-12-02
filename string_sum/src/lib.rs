use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
	Ok((a + b).to_string())
}

/// Python module implemented in Rust - must match `lib.name` in Cargo.toml
// with random fixes from https://stackoverflow.com/questions/79819313/no-method-named-add-function-found-for-reference-pyo3typespymodule
#[pymodule]
fn string_sum(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {//&PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(sum_as_string, &m)?)?;
	
	Ok(())
}
