use pyo3::{types::PyModule, FromPyObject, IntoPy, Py, PyAny, Python};

const PYMODULE: &str = "openff.units.units";

#[derive(FromPyObject)]
pub struct Unit(Py<PyAny>);

#[derive(FromPyObject)]
pub struct Quantity(pub Py<PyAny>);

impl Quantity {
    pub fn new(value: impl IntoPy<Py<PyAny>>, unit: Unit) -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            m.getattr("Quantity")
                .unwrap()
                .call1((value, unit.0))
                .unwrap()
                .extract()
                .unwrap()
        })
    }
}

#[derive(FromPyObject)]
pub struct UnitRegistry(pub(crate) Py<PyAny>);

fn get_defaults_path() -> String {
    Python::with_gil(|py| {
        let m = PyModule::import(py, PYMODULE).unwrap();
        m.getattr("get_defaults_path")
            .unwrap()
            .call0()
            .unwrap()
            .extract()
            .unwrap()
    })
}

pub fn unit() -> UnitRegistry {
    Python::with_gil(|py| {
        let m = PyModule::import(py, PYMODULE).unwrap();
        m.getattr("UnitRegistry")
            .unwrap()
            .call1((get_defaults_path(),))
            .unwrap()
            .extract()
            .unwrap()
    })
}

impl UnitRegistry {
    pub fn elementary_charge(&self) -> Unit {
        Python::with_gil(|py| {
            self.0
                .getattr(py, "elementary_charge")
                .unwrap()
                .extract(py)
                .unwrap()
        })
    }
}
