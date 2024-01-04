use pyo3::{
    types::{PyIterator, PyModule},
    FromPyObject, Py, PyAny, PyObject, Python,
};

use crate::{get_props, Molecule};

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

    pub fn from_molecules(molecules: Vec<Molecule>) -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            let top = m.getattr("Topology").unwrap();
            top.call_method1("from_molecules", (molecules,))
                .unwrap()
                .extract()
                .unwrap()
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

    get_props! {
        n_unique_molecules, usize;
        n_molecules, usize;
    }
}

#[cfg(test)]
mod tests {
    use openff_units::{unit, Quantity};

    use super::*;

    fn create_ethanol() -> Molecule {
        let mut ethanol = Molecule::new();
        ethanol.add_atom(6, 0, false);
        ethanol.add_atom(6, 0, false);
        ethanol.add_atom(8, 0, false);
        ethanol.add_atom(1, 0, false);
        ethanol.add_atom(1, 0, false);
        ethanol.add_atom(1, 0, false);
        ethanol.add_atom(1, 0, false);
        ethanol.add_atom(1, 0, false);
        ethanol.add_atom(1, 0, false);
        ethanol.add_bond(0, 1, 1, false, 1.33);
        ethanol.add_bond(1, 2, 1, false, 1.23);
        ethanol.add_bond(0, 3, 1, false, 1.0);
        ethanol.add_bond(0, 4, 1, false, 1.0);
        ethanol.add_bond(0, 5, 1, false, 1.0);
        ethanol.add_bond(1, 6, 1, false, 1.0);
        ethanol.add_bond(1, 7, 1, false, 1.0);
        ethanol.add_bond(2, 8, 1, false, 1.0);
        let charges = Quantity::new(
            vec![-0.4, -0.3, -0.2, -0.1, 0.00001, 0.1, 0.2, 0.3, 0.4],
            unit().elementary_charge(),
        );
        ethanol.set_partial_charges(charges);
        ethanol
    }

    #[test]
    fn unique_molecules() {
        let top =
            Topology::from_molecules(vec![create_ethanol(), create_ethanol()]);
        assert_eq!(top.n_unique_molecules(), 1);
        assert_eq!(top.n_molecules(), 2);
    }
}
