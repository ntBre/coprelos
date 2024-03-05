//! I've never constructed something in here directly, so I'm just skipping any
//! other features for now

use pyo3::{
    types::{IntoPyDict, PyList, PyModule},
    FromPyObject, Py, PyAny, PyObject, Python,
};
use utils::{get_props, set_props};

#[derive(FromPyObject)]
pub struct ParameterHandler(pub(crate) Py<PyAny>);

impl ParameterHandler {
    get_props! {
        parameters, Vec<Parameter>;
    }

    pub fn get_parameter(&self, id: &str) -> Option<Parameter> {
        Python::with_gil(|py| {
            let obj: PyObject = self
                .0
                .call_method1(
                    py,
                    "get_parameter",
                    ([("id", id)].into_py_dict(py),),
                )
                .unwrap();
            let obj: &PyList = obj.extract(py).unwrap();
            obj.get_item(0).ok().map(|item| item.extract().unwrap())
        })
    }

    pub fn clear_parameters(&mut self) {
        Python::with_gil(|py| {
            self.0
                .getattr(py, "parameters")
                .unwrap()
                .call_method0(py, "clear")
                .unwrap();
        })
    }

    pub fn add_parameter(&mut self, p: Parameter) {
        Python::with_gil(|py| {
            self.0
                .call_method(
                    py,
                    "add_parameter",
                    (),
                    Some([("parameter", p.0)].into_py_dict(py)),
                )
                .unwrap();
        })
    }
}

#[derive(FromPyObject)]
pub struct Parameter(pub(crate) Py<PyAny>);

impl std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{id}", id = self.id())
    }
}

impl Parameter {
    get_props! {
        id, String;
        smirks, String;
    }

    /// Return all of the parameter's force constants as a vector.
    pub fn k(&self) -> Vec<f64> {
        Python::with_gil(|py| {
            let fun =
                PyModule::from_code(py, include_str!("parameters.py"), "", "")
                    .unwrap()
                    .getattr("get_k")
                    .unwrap();
            fun.call1((&self.0,)).unwrap().extract().unwrap()
        })
    }

    set_props! {
        set_id => id;
    }
}

#[cfg(test)]
mod tests {
    use crate::ForceField;

    #[test]
    fn get_k() {
        let ff = ForceField::load("openff-2.1.0.offxml").unwrap();
        // torsions should straightforwardly be list/Vec
        let h = ff.get_parameter_handler("ProperTorsions").unwrap();
        let ps = h.parameters();
        assert_eq!(ps[0].k().len(), 1);
        assert_eq!(ps[8].k().len(), 2);
        // bonds and angles should need to be converted in `k`
        let h = ff.get_parameter_handler("Bonds").unwrap();
        let ps = h.parameters();
        assert_eq!(ps[0].k().len(), 1);
        assert_eq!(ps[8].k().len(), 1);
    }
}
