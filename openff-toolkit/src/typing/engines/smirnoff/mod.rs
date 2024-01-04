pub mod forcefield;

pub mod parameters {
    use pyo3::{FromPyObject, Py, PyAny};

    #[derive(FromPyObject)]
    pub struct ParameterHandler(pub(crate) Py<PyAny>);
}

pub mod io {
    use pyo3::{FromPyObject, Py, PyAny};

    #[derive(FromPyObject)]
    pub struct ParameterIOHandler(pub(crate) Py<PyAny>);
}
