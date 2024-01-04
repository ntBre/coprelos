use pyo3::{FromPyObject, Py, PyAny};

#[derive(FromPyObject)]
pub struct Molecule(pub(crate) Py<PyAny>);

#[derive(FromPyObject)]
pub struct Topology(pub(crate) Py<PyAny>);
