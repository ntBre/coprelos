use pyo3::{types::PyModule, FromPyObject, Py, PyAny, PyResult, Python};

const PYMODULE: &str = "openff.toolkit.typing.engines.smirnoff.forcefield";

#[derive(FromPyObject)]
pub struct ForceField(Py<PyAny>);

impl ForceField {
    /// Load a ForceField from one SMIRNOFF parameter definition file.
    pub fn load(path: &str) -> PyResult<Self> {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE)?;
            let ff = m.getattr("ForceField")?;
            // TODO handle kwargs, probably with a builder
            Ok(ff.call1((path,))?.extract()?)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_forcefield() {
        ForceField::load("openff-2.1.0.offxml").unwrap();
    }
}
