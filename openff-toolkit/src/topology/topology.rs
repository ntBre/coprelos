use pyo3::{
    types::{PyIterator, PyModule},
    FromPyObject, Py, PyAny, PyObject, Python,
};

use crate::Molecule;

const PYMODULE: &str = "openff.toolkit.topology.topology";

/// A chemical representation of a system containing one or more molecules
/// appearing in a specified order
#[derive(FromPyObject)]
pub struct Topology(pub(crate) Py<PyAny>);

impl Topology {
    pub fn new() -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            let top = m.getattr("Topology").unwrap();
            top.call0().unwrap().extract().unwrap()
        })
    }

    pub fn unique_molecules(&self) -> Vec<Molecule> {
        Python::with_gil(|py| {
            let p: PyObject =
                self.0.call_method0(py, "unique_molecules").unwrap();
            let p: &PyAny = p.as_ref(py);
            let iter = PyIterator::from_object(p).unwrap();
            let mut ret = Vec::new();
            for mol in iter.iter().unwrap().flatten() {
                ret.push(mol.extract().unwrap());
            }
            ret
        })
    }
}
