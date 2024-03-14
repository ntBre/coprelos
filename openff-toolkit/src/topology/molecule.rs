use openff_units::Quantity;
use pyo3::{
    types::{IntoPyDict, PyModule},
    FromPyObject, IntoPy, Py, PyAny, Python,
};

use crate::Topology;

const PYMODULE: &str = "openff.toolkit.topology.molecule";

#[derive(FromPyObject)]
pub struct Molecule(pub(crate) Py<PyAny>);

impl IntoPy<Py<PyAny>> for Molecule {
    fn into_py(self, _py: Python<'_>) -> Py<PyAny> {
        self.0
    }
}

impl Default for Molecule {
    fn default() -> Self {
        Self::new()
    }
}

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

    pub fn add_bond(
        &mut self,
        atom1: usize,
        atom2: usize,
        bond_order: usize,
        is_aromatic: bool,
        fractional_bond_order: f64,
    ) -> usize {
        Python::with_gil(|py| {
            let kwargs = [("fractional_bond_order", fractional_bond_order)]
                .into_py_dict(py);
            self.0
                .call_method(
                    py,
                    "add_bond",
                    (atom1, atom2, bond_order, is_aromatic),
                    Some(kwargs),
                )
                .unwrap()
                .extract(py)
                .unwrap()
        })
    }

    pub fn set_partial_charges(&mut self, charges: Quantity) {
        Python::with_gil(|py| {
            self.0.setattr(py, "partial_charges", charges.0).unwrap();
        })
    }

    /// Return a SMILES representation of `self`, optionally in isomeric,
    /// explicit-hydrogen, and mapped form. In Python, the default values are
    /// `isomeric=True`, `explicit_hydrogens=True`, and `mapped=False`. For
    /// these values, see [Molecule::to_smiles_default].
    pub fn to_smiles(
        &self,
        isomeric: bool,
        explicit_hydrogens: bool,
        mapped: bool,
    ) -> String {
        Python::with_gil(|py| {
            self.0
                .call_method1(
                    py,
                    "to_smiles",
                    (isomeric, explicit_hydrogens, mapped),
                )
                .unwrap()
                .extract(py)
                .unwrap()
        })
    }

    pub fn to_topology(&self) -> Topology {
        Python::with_gil(|py| {
            self.0
                .call_method0(py, "to_topology")
                .unwrap()
                .extract(py)
                .unwrap()
        })
    }

    pub fn to_smiles_default(&self) -> String {
        self.to_smiles(true, true, false)
    }
}
