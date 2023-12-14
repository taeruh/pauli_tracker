use lib::pauli::{
    Pauli,
    PauliEnum,
};
use pyo3::{
    types::PyModule,
    PyResult,
    Python,
};

/// Multiply two Paulis
#[pyo3::pyfunction]
fn product(ax: bool, az: bool, bx: bool, bz: bool) -> PyResult<String> {
    let mut a = PauliEnum::new_product(ax, az);
    let b = PauliEnum::new_product(bx, bz);
    a.add(b);
    Ok(a.to_string())
}

/// The Pauli Tracker
#[pyo3::pymodule]
fn pauli_tracker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(product, m)?)?;
    Ok(())
}
