use pyo3::{
    types::{IntoPyDict, PyModule},
    FromPyObject, IntoPy, Py, PyAny, Python,
};
use qcportal::PortalClient;

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
                    let m = PyModule::import(py, "openff.qcsubmit.results").unwrap();
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
        })*
    }
}

result_collection! {
    OptimizationResultCollection,
    TorsionDriveResultCollection,
}
