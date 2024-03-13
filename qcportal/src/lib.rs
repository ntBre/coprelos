//! Modeled after MolSSI's new QCPortal API

use pyo3::{types::PyModule, FromPyObject, IntoPy, Py, PyAny, Python};

#[derive(FromPyObject)]
pub struct PortalClient(Py<PyAny>);

impl PortalClient {
    pub fn new(address: &str) -> Self {
        Python::with_gil(|py| {
            let m = PyModule::import(py, "qcportal").unwrap();
            m.getattr("PortalClient")
                .unwrap()
                .call1((address,))
                .unwrap()
                .extract()
                .unwrap()
        })
    }
}

impl IntoPy<Py<PyAny>> for PortalClient {
    fn into_py(self, _py: Python<'_>) -> Py<PyAny> {
        self.0
    }
}

pub mod record_models {
    use pyo3::{types::PyModule, FromPyObject, IntoPy, Py, PyAny, Python};

    const PYMODULE: &str = "qcportal.record_models";

    pub enum RecordStatus {
        Cancelled,
        Complete,
        Deleted,
        Error,
        Invalid,
        Running,
        Waiting,
    }

    impl IntoPy<Py<PyAny>> for RecordStatus {
        fn into_py(self, py: Python<'_>) -> Py<PyAny> {
            let m = PyModule::import(py, PYMODULE).unwrap();
            let en = m.getattr("RecordStatusEnum").unwrap();
            let attr = match self {
                RecordStatus::Complete => "complete",
                RecordStatus::Cancelled => "cancelled",
                RecordStatus::Deleted => "deleted",
                RecordStatus::Error => "error",
                RecordStatus::Invalid => "invalid",
                RecordStatus::Running => "running",
                RecordStatus::Waiting => "waiting",
            };
            en.getattr(attr).unwrap().into()
        }
    }

    #[derive(Clone, FromPyObject)]
    pub struct TorsiondriveKeywords {
        pub dihedrals: Vec<(usize, usize, usize, usize)>,
    }

    #[derive(Clone, FromPyObject)]
    pub struct TorsiondriveSpecification {
        pub keywords: TorsiondriveKeywords,
    }

    #[derive(Clone, FromPyObject)]
    pub struct TorsiondriveRecord {
        pub specification: TorsiondriveSpecification,
    }
}
