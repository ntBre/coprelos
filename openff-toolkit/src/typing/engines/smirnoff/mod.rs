//! In many cases here, only the base classes are implemented because the inner
//! PyAny will call the correct implementation on its own. Eventually it might
//! be useful to define the base classes as traits and the concrete classes as
//! structs, but this is a simpler start

pub mod forcefield;
pub mod parameters;

pub mod io {
    use pyo3::{FromPyObject, Py, PyAny};

    #[derive(FromPyObject)]
    pub struct ParameterIOHandler(pub(crate) Py<PyAny>);
}
