use pyo3::ffi;
use pyo3::prelude::*;

extern "C" {
    fn PyInit_gui() -> *mut ffi::PyObject;
}

pub fn as_submodule_of(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Python::with_gil(|py| unsafe {
        let gui = Bound::from_owned_ptr_or_err(py, PyInit_gui())?;
        let sub = gui.downcast::<PyModule>()?;
        m.add_submodule(&sub)?;
        Ok(())
    })
}
