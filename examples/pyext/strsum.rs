use pyo3::prelude::*;

#[pymodule]
fn strsum(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfunction]
    #[pyo3(name = "sum_as_string")]
    fn some_function(_py: Python, a: i64, b: i64) -> PyResult<String> {
        let r = sum_as_string(a, b);
        Ok(r)
    }
    m.add_function(wrap_pyfunction!(some_function, m)?)?;
    Ok(())
}

fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a+b)
}
