use pyo3::prelude::*;
use pyo3::ffi;

extern "C" {
    fn PyInit_gui() -> *mut ffi::PyObject;
}

pub fn get<'p>(py: Python<'p>) -> PyResult<&'p PyModule> {
    unsafe {
        let gui = PyInit_gui();
        py.from_owned_ptr_or_err(gui)
    }
}
