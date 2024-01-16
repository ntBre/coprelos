use pyo3::{FromPyObject, Py, PyAny};

#[allow(unused)]
#[derive(FromPyObject)]
pub struct Interchange(pub(crate) Py<PyAny>);
