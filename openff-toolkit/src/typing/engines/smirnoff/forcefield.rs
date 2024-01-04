use pyo3::{
    types::PyModule, FromPyObject, IntoPy, Py, PyAny, PyResult, Python,
};

use super::{io::ParameterIOHandler, parameters::ParameterHandler};

const PYMODULE: &str = "openff.toolkit.typing.engines.smirnoff.forcefield";

pub fn get_available_force_fields() -> Vec<String> {
    Python::with_gil(|py| {
        let m = PyModule::import(py, PYMODULE).unwrap();
        m.call_method0("get_available_force_fields")
            .unwrap()
            .extract()
            .unwrap()
    })
}

#[derive(FromPyObject)]
pub struct ForceField(Py<PyAny>);

macro_rules! getters {
    ($($method_name:ident, $return_ty:ty$(;)*)*) => {
        $(pub fn $method_name(&self) -> $return_ty {
            Python::with_gil(|py| {
                self.0
                .getattr(py, stringify!($method_name))
                .unwrap()
                .extract(py)
                .unwrap()
            })
        })*
    }
}

macro_rules! setters {
    ($($method_name:ident => $py_method_name:ident$(;)*)*) => {
        $(pub fn $method_name(&mut self, val: impl IntoPy<Py<PyAny>>) {
            Python::with_gil(|py| {
                self.0
                .setattr(py, stringify!($py_method_name), val)
                .unwrap()
            })
        })*
    }
}

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

    getters! {
        aromaticity_model, String;
        author, String;
        date, String;
        registered_parameter_handlers, Vec<String>;
    }

    setters! {
        set_aromaticity_model => aromaticity_model;
        set_author => author;
        set_date => date;
    }

    pub fn register_parameter_handler(&mut self, ph: ParameterHandler) {
        Python::with_gil(|py| {
            self.0
                .call_method1(py, "register_parameter_handler", (ph.0,))
                .unwrap();
        })
    }

    pub fn register_parameter_io_handler(&mut self, ph: ParameterIOHandler) {
        Python::with_gil(|py| {
            self.0
                .call_method1(py, "register_parameter_handler", (ph.0,))
                .unwrap();
        })
    }

    pub fn get_parameter_handler(
        &self,
        tagname: &str,
    ) -> PyResult<ParameterHandler> {
        Python::with_gil(|py| {
            Ok(self
                .0
                .call_method1(py, "get_parameter_handler", (tagname,))?
                .extract(py)?)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::constants::DEFAULT_AROMATICITY_MODEL;

    use super::*;

    #[test]
    fn avail() {
        let got = get_available_force_fields();
        assert!(got.contains(&String::from("openff-2.1.0.offxml")));
    }

    #[test]
    fn load_forcefield() {
        ForceField::load("openff-2.1.0.offxml").unwrap();
    }

    #[test]
    fn get_aromaticity_model() {
        let mdl = ForceField::load("openff-2.1.0.offxml")
            .unwrap()
            .aromaticity_model();
        assert_eq!(mdl, DEFAULT_AROMATICITY_MODEL);
    }

    #[test]
    fn get_parameter_handler() {
        let ff = ForceField::load("openff-2.1.0.offxml").unwrap();
        ff.get_parameter_handler("Bonds").unwrap();
        ff.get_parameter_handler("Angles").unwrap();
        ff.get_parameter_handler("ProperTorsions").unwrap();
        ff.get_parameter_handler("ImproperTorsions").unwrap();
    }

    #[test]
    fn getters_run() {
        let ff = ForceField::load("openff-2.1.0.offxml").unwrap();
        ff.author();
        ff.date();
        ff.registered_parameter_handlers();
    }

    #[test]
    fn setters_run() {
        let mut ff = ForceField::load("openff-2.1.0.offxml").unwrap();
        ff.set_aromaticity_model(DEFAULT_AROMATICITY_MODEL);
        ff.set_author("bwestbro");
        ff.set_date("today");
    }
}
