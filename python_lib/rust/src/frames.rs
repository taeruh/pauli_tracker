use std::mem;

use lib::{pauli, tracker::frames::induced_order};
use pyo3::{PyResult, Python, types::PyModuleMethods};

use crate::{BitVec, Module, impl_helper::serialization, pauli::PauliStack};

#[pyo3::pyclass(subclass)]
/// Compare `PartialOrderGraph
/// <https://docs.rs/pauli_tracker/latest/pauli_tracker/tracker/frames/induced_order/type.PartialOrderGraph.html>`_.
/// Use :meth:`into_py_graph` to turn it into a Python type.
pub struct PartialOrderGraph(pub induced_order::PartialOrderGraph);

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
    #[pyo3(text_signature = "(self, graph)")]
    fn __init__(&self, _graph: induced_order::PartialOrderGraph) {}

    #[doc = crate::transform!()]
    ///
    /// Returns:
    ///     list[list[tuple[int, list[int]]]]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_graph(&self) -> induced_order::PartialOrderGraph {
        self.0.clone()
    }

    #[doc = crate::take_transform!()]
    ///
    /// Returns:
    ///     list[list[tuple[int, list[int]]]]:
    fn take_into_py_graph(&mut self) -> induced_order::PartialOrderGraph {
        mem::take(&mut self.0)
    }
}

serialization::serde!(PartialOrderGraph);

#[pyo3::pyclass(subclass)]
/// The frames of a `Frames`-tracker with swapped major and minor axis.
///
/// This is usually returned from the according `stacked_transpose` method of a
/// `Frames` object. The frames are now on the major axis and the qubits on the minor
/// axis.
#[derive(Clone)]
struct StackedTransposed(Vec<pauli::PauliStack<BitVec>>);

#[pyo3::pymethods]
impl StackedTransposed {
    #[new]
    fn __new__(stacks: Vec<PauliStack>) -> Self {
        Self(stacks.into_iter().map(|s| s.0).collect())
    }

    /// Create a new StackedTransposed
    ///
    /// Args:
    ///     stacks (list[PauliStack]): The stacks to wrap.
    ///
    /// Returns:
    ///     StackedTransposed
    #[pyo3(text_signature = "(self, stacks)")]
    fn __init__(&self, _stacks: Vec<PauliStack>) {}

    /// Get the Pauli stack at the given index.
    ///
    /// Use :meth:`get_and_add_to_stack` if you directly want to add it to another stack
    /// to avoid cloning.
    ///
    /// Args:
    ///     index (int): The index of the stack to get.
    ///
    /// Returns:
    ///     PauliStack:
    fn get(&self, index: usize) -> Option<PauliStack> {
        self.0.get(index).cloned().map(PauliStack)
    }

    /// Get the Pauli stack at the given index and add it to the given stack.
    ///
    /// Args:
    ///     index (int): The index of the stack to get.
    ///     stack (PauliStack): The stack to add the gotten stack to.
    fn get_and_add_to_stack(&self, index: usize, stack: &mut PauliStack) {
        stack.0.xor_inplace(self.0.get(index).unwrap());
    }

    fn pop(&mut self) -> Option<PauliStack> {
        self.0.pop().map(PauliStack)
    }

    #[doc = crate::transform!()]
    ///
    /// Returns:
    ///     list[tuple[list[int], list[int]]]
    #[allow(clippy::wrong_self_convention)]
    fn into_py_matrix(&self) -> Vec<(Vec<u64>, Vec<u64>)> {
        into_py_matrix(self.0.clone())
    }

    #[doc = crate::take_transform!()]
    ///
    /// Returns:
    ///     list[tuple[list[int], list[int]]]
    fn take_into_py_matrix(&mut self) -> Vec<(Vec<u64>, Vec<u64>)> {
        into_py_matrix(mem::take(&mut self.0))
    }
}

fn into_py_matrix(stacks: Vec<pauli::PauliStack<BitVec>>) -> Vec<(Vec<u64>, Vec<u64>)> {
    stacks.into_iter().map(|s| PauliStack(s).into_py_tuple()).collect()
}

serialization::serde!(StackedTransposed);

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

            /// Get the frames, but with swapped major and minor axis and sorted bits.
            ///
            /// The frames are now on the major axis and the qubits on the minor axis.
            ///
            /// Consider using :func:`take_stacked_transpose` to avoid cloning.
            ///
            /// Args:
            ///     highest_qubit (int): The highest qubit index that has been tracked.
            ///
            /// Returns:
            ///     StackedTransposed:
            fn stacked_transpose(
                &self,
                highest_qubit: usize,
            ) -> crate::frames::StackedTransposed {
                crate::frames::StackedTransposed(
                    self.0.clone().stacked_transpose(highest_qubit),
                )
            }

            /// Like :func:`stacked_transpose`, but take out the internal data (replacing
            /// it with its default value).
            fn take_stacked_transpose(
                &mut self,
                highest_qubit: usize,
            ) -> crate::frames::StackedTransposed {
                crate::frames::StackedTransposed(
                    mem::take(&mut self.0).stacked_transpose(highest_qubit),
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
    module.pymodule.add_class::<PartialOrderGraph>()?;
    module.pymodule.add_class::<StackedTransposed>()?;
    parent_module.add_submodule(py, module)?;
    Ok(())
}
