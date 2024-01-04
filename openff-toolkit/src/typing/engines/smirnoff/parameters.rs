//! I've never constructed something in here directly, so I'm just skipping any
//! other features for now

use pyo3::{FromPyObject, Py, PyAny};

#[derive(FromPyObject)]
pub struct ParameterHandler(pub(crate) Py<PyAny>);
