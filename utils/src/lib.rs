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

/// Generate implementations of IntoPy<Py<PyAny>> and AsRef<PyAny> for a tuple
/// struct containing a Py<PyAny> as its first field
#[macro_export]
macro_rules! into_py {
    ($($struct:ident$(,)?)*) => {
        $(impl pyo3::IntoPy<pyo3::Py<pyo3::PyAny>> for $struct {
            fn into_py(self, _py: Python<'_>) -> Py<PyAny> {
                self.0
            }
        })*
        $(impl AsRef<PyAny> for $struct {
            fn as_ref(&self) -> &PyAny {
                Python::with_gil(|py| self.0.as_ref(py))
            }
        })*

    }
}
