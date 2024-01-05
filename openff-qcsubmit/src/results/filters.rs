use pyo3::{
    types::{IntoPyDict, PyModule},
    FromPyObject, IntoPy, Py, PyAny, Python,
};
use qcportal::record_models::RecordStatus;

const PYMODULE: &str = "openff.qcsubmit.results.filters";

#[derive(FromPyObject)]
pub struct ConformerRMSDFilter(Py<PyAny>);

impl ConformerRMSDFilter {
    pub fn new(max_conformers: usize) -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            let kwargs = [("max_conformers", max_conformers)].into_py_dict(py);
            m.getattr("ConformerRMSDFilter")
                .unwrap()
                .call((), Some(kwargs))
                .unwrap()
                .extract()
                .unwrap()
        })
    }
}

#[derive(FromPyObject)]
pub struct ConnectivityFilter(Py<PyAny>);

impl ConnectivityFilter {
    pub fn new(tolerance: f64) -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            let kwargs = [("tolerance", tolerance)].into_py_dict(py);
            m.getattr("ConnectivityFilter")
                .unwrap()
                .call((), Some(kwargs))
                .unwrap()
                .extract()
                .unwrap()
        })
    }
}

#[derive(FromPyObject)]
pub struct ElementFilter(Py<PyAny>);

impl ElementFilter {
    pub fn new(elements: Vec<&str>) -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            let kwargs = [("allowed_elements", elements)].into_py_dict(py);
            m.getattr("ElementFilter")
                .unwrap()
                .call((), Some(kwargs))
                .unwrap()
                .extract()
                .unwrap()
        })
    }
}

#[derive(FromPyObject)]
pub struct HydrogenBondFilter(Py<PyAny>);

#[derive(FromPyObject)]
pub struct RecordStatusFilter(Py<PyAny>);

impl RecordStatusFilter {
    pub fn new(status: RecordStatus) -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            let kwargs = [("status", status.into_py(py))].into_py_dict(py);
            m.getattr("RecordStatusFilter")
                .unwrap()
                .call((), Some(kwargs))
                .unwrap()
                .extract()
                .unwrap()
        })
    }
}

#[derive(FromPyObject)]
pub struct ResultRecordFilter(Py<PyAny>);

#[derive(FromPyObject)]
pub struct UnperceivableStereoFilter(Py<PyAny>);

impl UnperceivableStereoFilter {
    pub fn new() -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, PYMODULE).unwrap();
            m.getattr("UnperceivableStereoFilter")
                .unwrap()
                .call0()
                .unwrap()
                .extract()
                .unwrap()
        })
    }
}

pub trait Filter {
    fn as_py(&self) -> Py<PyAny>;
}

impl Filter for ConformerRMSDFilter {
    fn as_py(&self) -> Py<PyAny> {
        self.0.clone()
    }
}
impl Filter for ConnectivityFilter {
    fn as_py(&self) -> Py<PyAny> {
        self.0.clone()
    }
}
impl Filter for ElementFilter {
    fn as_py(&self) -> Py<PyAny> {
        self.0.clone()
    }
}
impl Filter for HydrogenBondFilter {
    fn as_py(&self) -> Py<PyAny> {
        self.0.clone()
    }
}
impl Filter for RecordStatusFilter {
    fn as_py(&self) -> Py<PyAny> {
        self.0.clone()
    }
}
impl Filter for ResultRecordFilter {
    fn as_py(&self) -> Py<PyAny> {
        self.0.clone()
    }
}
impl Filter for UnperceivableStereoFilter {
    fn as_py(&self) -> Py<PyAny> {
        self.0.clone()
    }
}
