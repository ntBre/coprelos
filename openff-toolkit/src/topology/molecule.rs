use pyo3::{types::PyModule, FromPyObject, Py, PyAny, Python};

const PYMODULE: &str = "openff.toolkit.topology.molecule";

#[derive(FromPyObject)]
pub struct Molecule(pub(crate) Py<PyAny>);

impl Molecule {
    pub fn new() -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            let top = m.getattr("Molecule").unwrap();
            top.call0().unwrap().extract().unwrap()
        })
    }

    pub fn add_atom(
        &mut self,
        atomic_number: u8,
        formal_charge: i8,
        is_aromatic: bool,
    ) -> usize {
        Python::with_gil(|py| {
            self.0
                .call_method1(
                    py,
                    "add_atom",
                    (atomic_number, formal_charge, is_aromatic),
                )
                .unwrap()
                .extract(py)
                .unwrap()
        })
    }
}
