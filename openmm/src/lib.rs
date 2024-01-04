use pyo3::{FromPyObject, Py, PyAny};

#[derive(FromPyObject)]
pub struct System(pub(crate) Py<PyAny>);
