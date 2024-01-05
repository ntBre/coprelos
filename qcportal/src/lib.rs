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
