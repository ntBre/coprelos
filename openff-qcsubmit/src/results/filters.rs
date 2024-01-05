use pyo3::{
    types::{IntoPyDict, PyModule},
    FromPyObject, IntoPy, Py, PyAny, Python,
};
use qcportal::record_models::RecordStatus;

use super::BaseResultCollection;

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

pub trait Filter<T: BaseResultCollection> {
    fn apply(&self, dataset: T) -> T;
}

fn apply<
    F: IntoPy<Py<PyAny>>,
    T: BaseResultCollection + for<'a> FromPyObject<'a> + IntoPy<Py<PyAny>>,
>(
    filter: F,
    dataset: T,
) -> T {
    Python::with_gil(|py| {
        filter
            .into_py(py)
            .call_method1(py, "apply", (dataset,))
            .unwrap()
            .extract(py)
            .unwrap()
    })
}

macro_rules! make_filter {
    ($($struct:ident$(,)?)*) => {
        $(impl<T> Filter<T> for $struct
        where
        T: BaseResultCollection + IntoPy<Py<PyAny>> + for<'a> FromPyObject<'a>,
        {
            fn apply(&self, dataset: T) -> T {
                apply(&self.0, dataset)
            }
        })*
    }
}

make_filter! {
    ConformerRMSDFilter,
    ConnectivityFilter,
    ElementFilter,
    HydrogenBondFilter,
    RecordStatusFilter,
    ResultRecordFilter,
    UnperceivableStereoFilter,
}
