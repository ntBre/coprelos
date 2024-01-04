use pyo3::{FromPyObject, Py, PyAny};

#[derive(FromPyObject)]
pub struct Interchange(pub(crate) Py<PyAny>);
