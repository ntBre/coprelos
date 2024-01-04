//! I've never constructed something in here directly, so I'm just skipping any
//! other features for now

use pyo3::{FromPyObject, Py, PyAny};

use crate::get_props;

#[derive(FromPyObject)]
pub struct ParameterHandler(pub(crate) Py<PyAny>);

impl ParameterHandler {
    get_props! {
        parameters, Vec<Parameter>;
    }
}

#[derive(FromPyObject)]
pub struct Parameter(pub(crate) Py<PyAny>);

impl Parameter {
    get_props! {
        id, String;
    }
}
