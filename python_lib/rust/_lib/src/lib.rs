use pyo3::{Bound, PyResult, Python, types::PyModule};

#[pyo3::pymodule]
fn _lib(py: Python, module: Bound<'_, PyModule>) -> PyResult<()> {
    pauli_tracker_pyo3::create_module(py, module)
}
