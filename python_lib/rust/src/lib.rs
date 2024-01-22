use pyo3::{types::PyModule, PyResult, Python};

struct Module<'m> {
    pymodule: &'m PyModule,
    path: String,
}

impl<'m> Module<'m> {
    fn new(py: Python<'m>, name: &str, mut path: String) -> PyResult<Self> {
        path.push_str(format!(".{}", name).as_str());
        Ok(Self {
            pymodule: PyModule::new(py, name)?,
            path,
        })
    }

    fn add_submodule(&self, py: Python<'_>, submodule: Self) -> PyResult<()> {
        self.pymodule.add_submodule(submodule.pymodule)?;
        py.import("sys")?
            .getattr("modules")?
            .set_item(submodule.path, submodule.pymodule)?;
        Ok(())
    }
}

mod impl_helper;

mod frames;
mod live;
mod pauli;

// ensuring that we always use 64 bits per chunk (promised by the API docs and used
// internally, e.g., in the bitvector_to_boolvector function in __init__.py)
type BitVec = bitvec::vec::BitVec<u64, bitvec::order::LocalBits>;

#[pyo3::pymodule]
fn _lib(py: Python, module: &PyModule) -> PyResult<()> {
    let module = Module {
        pymodule: module,
        path: "pauli_tracker._lib".to_string(),
    };
    live::add_module(py, &module)?;
    frames::add_module(py, &module)?;
    pauli::add_module(py, &module)?;
    Ok(())
}
