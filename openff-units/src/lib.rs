use pyo3::{FromPyObject, Py, PyAny};

#[derive(FromPyObject)]
pub struct Quantity(pub(crate) Py<PyAny>);

#[derive(FromPyObject)]
pub struct UnitRegistry(pub(crate) Py<PyAny>);

pub fn unit() -> UnitRegistry {
    todo!();
}
