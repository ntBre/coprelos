use std::collections::HashMap;
use std::path::Path;

use pyo3::{
    types::{IntoPyDict, PyModule},
    FromPyObject, IntoPy, Py, PyAny, Python,
};
use qcportal::PortalClient;
use utils::{get_props, into_py};

use filters::Filter;

pub mod filters;

const PYMODULE: &str = "openff.qcsubmit.results";

pub trait BaseResultCollection {}

#[derive(Clone, FromPyObject)]
pub struct Entry(Py<PyAny>);

impl IntoPy<Py<PyAny>> for Entry {
    fn into_py(self, _py: Python<'_>) -> Py<PyAny> {
        self.0
    }
}

impl Entry {
    get_props! {
        record_id, usize;
    }
}

macro_rules! result_collection {
($($name:ident$(,)?)*) => {
    $(#[derive(FromPyObject)]
    pub struct $name(Py<PyAny>);

    impl $name {
        pub fn from_server(
            client: PortalClient,
            datasets: Vec<&str>,
            spec_name: &str,
        ) -> Self {
            Python::with_gil(|py| {
                let m = PyModule::import(py, PYMODULE).unwrap();
                let kwargs = [("client", client.into_py(py))].into_py_dict(py);
                kwargs.set_item("datasets", datasets).unwrap();
                kwargs.set_item("spec_name", spec_name).unwrap();
                m.getattr(stringify!($name))
                .unwrap()
                .call_method("from_server", (), Some(kwargs))
                .unwrap()
                .extract()
                .unwrap()
            })
        }

        pub fn parse_file(filename: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
            Python::with_gil(|py| {
                let m = PyModule::import(py, "openff.qcsubmit.results").unwrap();
                Ok(m.getattr(stringify!($name))?
                    .call_method1("parse_file", (filename.as_ref(),))?
                    .extract()?)
            })
        }

        pub fn json(&self, indent: usize) -> String {
            Python::with_gil(|py| {
                self.0
                .call_method(
                    py,
                    "json",
                    (),
                    Some([("indent", indent)].into_py_dict(py)),
                )
                .unwrap()
                .extract(py)
                .unwrap()
            })
        }

        pub fn entries(&self) -> HashMap<String, Vec<Entry>> {
            Python::with_gil(|py| {
                self.0
                .getattr(py, "entries")
                .unwrap()
                .extract(py)
                .unwrap()
            })
        }

        pub fn set_entries(&mut self, entries: HashMap<String, Vec<Entry>>) {
            Python::with_gil(|py| {
                self.0
                .setattr(py, "entries", entries.into_py(py))
                .unwrap();
            })
        }

        /// apply `filters` to the entries in `self` and overwrite self with the
        /// results
        pub fn filter(mut self, filters: &[Box<dyn Filter<$name>>]) -> Self {
            for filter in filters {
                self = filter.apply(self);
            }
            self
        }
    })*
}
}

result_collection! {
    OptimizationResultCollection,
    TorsionDriveResultCollection,
}

into_py! {
    OptimizationResultCollection,
    TorsionDriveResultCollection,
}

impl BaseResultCollection for OptimizationResultCollection {}
impl BaseResultCollection for TorsionDriveResultCollection {}
