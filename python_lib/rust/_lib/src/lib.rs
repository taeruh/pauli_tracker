use pyo3::{types::PyModule, PyResult, Python};

#[pyo3::pymodule]
fn _lib(py: Python, module: &PyModule) -> PyResult<()> {
    pauli_tracker_pyo3::create_module(py, module)
}
