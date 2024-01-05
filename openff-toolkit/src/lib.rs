pub mod topology;
pub mod typing;
pub mod utils;

pub use openff_units::{unit, Quantity};
pub use topology::{Molecule, Topology};
pub use typing::engines::smirnoff::forcefield::{
    get_available_force_fields, ForceField,
};

/// Generate methods on `self` to retrieve Python properties with the same name
/// and with type `return_ty`.
#[macro_export]
macro_rules! get_props {
    ($($method_name:ident, $return_ty:ty$(;)*)*) => {
        $(pub fn $method_name(&self) -> $return_ty {
            pyo3::Python::with_gil(|py| {
                self.0
                .getattr(py, stringify!($method_name))
                .unwrap()
                .extract(py)
                .unwrap()
            })
        })*
    }
}

/// Generate methods on `self` to set Python properties with the name and
/// `py_method_name`.
#[macro_export]
macro_rules! set_props {
    ($($method_name:ident => $py_method_name:ident$(;)*)*) => {
        $(pub fn $method_name(&mut self, val: impl pyo3::IntoPy<pyo3::Py<pyo3::PyAny>>) {
            Python::with_gil(|py| {
                self.0
                .setattr(py, stringify!($py_method_name), val)
                .unwrap()
            })
        })*
    }
}
