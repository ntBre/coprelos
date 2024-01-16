use pyo3::{FromPyObject, Py, PyAny};

#[allow(unused)]
#[derive(FromPyObject)]
pub struct System(pub(crate) Py<PyAny>);
