use lib::{pauli, tracker::frames::induced_order};
use pyo3::{PyResult, Python};

use crate::{
    impl_helper::{doc, serialization},
    pauli::PauliStack,
    BitVec, Module,
};

#[pyo3::pyclass(subclass)]
/// This is just `PartialOrderGraph
/// <https://docs.rs/pauli_tracker/latest/pauli_tracker/tracker/frames/induced_order/type.PartialOrderGraph.html>`_.
/// Use :meth:`into_py_graph` to turn it into a Python type.
struct PartialOrderGraph(pub induced_order::PartialOrderGraph);

#[pyo3::pymethods]
impl PartialOrderGraph {
    #[new]
    fn __new__(graph: induced_order::PartialOrderGraph) -> Self {
        Self(graph)
    }

    /// Create a new PartialOrderGraph.
    ///
    /// Args:
    ///     graph (list[list[tuple[int, list[int]]]]): The graph to wrap.
    ///
    /// Returns:
    ///     PartialOrderGraph:
    fn __init__(&self, _graph: induced_order::PartialOrderGraph) {}

    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     list[list[tuple[int, list[int]]]]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_graph(&self) -> induced_order::PartialOrderGraph {
        self.0.clone()
    }
}

serialization::serde!(PartialOrderGraph);

#[pyo3::pyclass(subclass)]
#[derive(Clone)]
struct StackedTransposedReverted(Vec<pauli::PauliStack<BitVec>>);

#[pyo3::pymethods]
impl StackedTransposedReverted {
    #[new]
    fn __new__(stacks: Vec<PauliStack>) -> Self {
        Self(stacks.into_iter().map(|s| s.0).collect())
    }

    /// Create a new StackedTransposedReverted.
    ///
    /// Args:
    ///     stacks (list[PauliStack]): The stacks to wrap.
    ///
    /// Returns:
    ///     StackedTransposedReverted:
    fn __init__(&self, _stack: Vec<PauliStack>) {}

    // Use get_and_add_to_stack if you directly want to add it to another stack to avoild
    // cloning.
    fn get(&self, index: usize) -> Option<PauliStack> {
        self.0.get(index).cloned().map(PauliStack)
    }

    /// Add the Pauli stack at the given index to the given stack.
    fn get_and_add_to_stack(&self, index: usize, stack: &mut PauliStack) {
        stack.0.xor_inplace(self.0.get(index).unwrap());
    }

    fn pop(&mut self) -> Option<PauliStack> {
        self.0.pop().map(PauliStack)
    }

    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     list[tuple[list[int], list[int]]]
    #[allow(clippy::wrong_self_convention)]
    fn into_py_matrix(&self) -> Vec<(Vec<u64>, Vec<u64>)> {
        self.0
            .clone()
            .into_iter()
            .map(|s| PauliStack(s).into_py_tuple())
            .collect()
    }
}

serialization::serde!(StackedTransposedReverted);

// Tracker and Init must be in scope for the macro to work.
macro_rules! impl_frames {
    ($storage:ty, $gentype:expr) => {
        type LibFrames = lib::tracker::frames::Frames<$storage>;

        #[doc = $gentype]
        #[pyo3::pyclass(subclass)]
        pub struct Frames(pub LibFrames);

        #[pyo3::pymethods]
        impl Frames {
            #[new]
            #[pyo3(signature = (len=0))]
            fn __new__(len: usize) -> Self {
                Self(LibFrames::init(len))
            }

            /// Create a new Frames tracker.
            ///
            /// Args:
            ///     len (int): The number of qubits to track
            ///
            /// Returns:
            ///     Frames:
            #[pyo3(text_signature = "(self, len=0)")]
            fn __init__(&self, _len: usize) {}

            /// Create a new qubit in the tracker, returning the old Pauli stack if the
            /// qubit was already initialized.
            fn new_qubit(&mut self, bit: usize) -> Option<crate::pauli::PauliStack> {
                self.0.new_qubit(bit).map(crate::pauli::PauliStack)
            }

            /// Remove a qubit in the tracker, returning the according Pauli stack and
            /// erroring if the qubit was not initialized.
            fn measure(
                &mut self,
                bit: usize,
            ) -> pyo3::PyResult<crate::pauli::PauliStack> {
                match self.0.measure(bit) {
                    Ok(p) => Ok(crate::pauli::PauliStack(p)),
                    Err(b) => {
                        Err(pyo3::exceptions::PyValueError::new_err(format!("{b}")))
                    },
                }
            }

            fn stacked_transpose_reverted(
                &self,
                num_qubits: usize,
            ) -> crate::frames::StackedTransposedReverted {
                crate::frames::StackedTransposedReverted(
                    self.0.clone().stacked_transpose_reverted(num_qubits),
                )
            }

            /// Get the Pauli stack of a qubit in the tracker, returning None if the
            /// qubit was not initialized. Note that this clones the data.
            fn get(&self, bit: usize) -> Option<crate::pauli::PauliStack> {
                self.0.get(bit).map(|p| crate::pauli::PauliStack(p.clone()))
            }

            /// This is just get_order_ as a method.
            ///
            /// If you directly want to turn it into a Python type, use
            /// :func:`get_py_order`, because this avoids cloning the
            /// graph (which would happen when calling
            /// :func:`~pauli_tracker.frames.PartialOrderGraph.into_py_graph`).
            ///
            /// Returns:
            ///     PartialOrderGraph:
            ///
            /// .. _get_order:
            ///    https://docs.rs/pauli_tracker/latest/pauli_tracker/tracker/frames/induced_order/fn.get_order.html
            fn get_order(&self, map: Vec<usize>) -> crate::frames::PartialOrderGraph {
                crate::frames::PartialOrderGraph(
                    lib::tracker::frames::induced_order::get_order(
                        lib::collection::Iterable::iter_pairs(self.0.as_storage()),
                        &map,
                    ),
                )
            }

            /// Like :func:`get_order`, but directly returns the graph as
            /// a Python type.
            ///
            /// Returns:
            ///     list[list[tuple[int, list[int]]]]:
            fn get_py_order(
                &self,
                map: Vec<usize>,
            ) -> lib::tracker::frames::induced_order::PartialOrderGraph {
                lib::tracker::frames::induced_order::get_order(
                    lib::collection::Iterable::iter_pairs(self.0.as_storage()),
                    &map,
                )
            }
        }

        crate::impl_helper::tracker::tracker_impl!(Frames);
        crate::impl_helper::serialization::serde!(Frames);
    };
}

pub mod map;
pub mod vec;

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "frames", parent_module.path.clone())?;
    map::add_module(py, &module)?;
    vec::add_module(py, &module)?;
    module.add_class::<PartialOrderGraph>()?;
    parent_module.add_submodule(py, module)?;
    Ok(())
}
