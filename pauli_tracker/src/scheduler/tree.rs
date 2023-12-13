/*!
Tools to make create scheduling paths easier.

Creating different scheduling paths is a little bit like traversing a tree, where the
nodes describe the set of qubits that are going to be measured in this step. The traits
and data structures are described in terms of a tree.
*/

use std::mem;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;

/// A trait which acts a little bit like a [zipper](https://wiki.haskell.org/Zipper).
pub trait Focus<I> {
    /// An error that can happen when focusing.
    type Error;

    /// Like [focus](Self::focus), but mutates `self` in place.
    fn focus_inplace(&mut self, instruction: I) -> Result<(), Self::Error>;

    /// Focus on the node described by `instruction`/measureable_set.
    // often just {let mut new = self.clone(); new.focus_inplace(instruction)?; Ok(new)}
    // but we don't want the Clone bound here
    fn focus(&mut self, instruction: I) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// A trait similar to [Focus], but instead of taking an instruction, it iterates over
/// all instructions.
pub trait FocusIterator {
    /// A byproduct when focusing, e.g., a list of qubits that are going to be measured.
    type IterItem;
    /// A value that can be returned after focusing a leaf node.
    type LeafItem;

    /// Focus on the next node.
    fn next_and_focus(&mut self) -> Option<(Self, Self::IterItem)>
    where
        Self: Sized;

    /// Check whether the current node is a leaf node. If true, return an associated
    /// item.
    fn at_leaf(&self) -> Option<Self::LeafItem>;
}

/// This struct can be used to describe whether one traverses forward in the tree,
/// focusing on a next node, or goes backward, to the previous state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Step<F, B> {
    /// Go forward.
    Forward(F),
    /// Go backward.
    Backward(B),
}

#[doc = non_semantic_default!()]
impl<F: Default, B> Default for Step<F, B> {
    fn default() -> Self {
        Step::Forward(F::default())
    }
}

/// An [Iterator] to sweep through the whole tree. To reduce the runtime, the iterator
/// keeps track of previous states in a stack, however, note that this requires more
/// memory.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sweep<T> {
    current: T,
    stack: Vec<T>,
}

/// This error might occur when trying to skipping a node in
/// [skip_current](Sweep::skip_current). This is usually not a problem, but rather a
/// final break condition in a loop.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("the stack is empty")]
pub struct EmptyStack;

impl<T> Sweep<T> {
    /// Initialize the iterator with a given state.
    pub fn new(current: T) -> Self {
        Self { current, stack: Vec::new() }
    }

    /// Get a reference to the current state.
    pub fn current(&self) -> &T {
        &self.current
    }

    /// Get a reference to the stack of tracked states.
    pub fn stack(&self) -> &Vec<T> {
        &self.stack
    }

    /// Skip traversing the tree from the current node. The current node is set to the
    /// last node in the stack. Errors if the stack is empty.
    pub fn skip_current(&mut self) -> Result<(), EmptyStack> {
        self.current = self.stack.pop().ok_or(EmptyStack)?;
        Ok(())
    }
}

impl<T: FocusIterator> Iterator for Sweep<T> {
    type Item = Step<T::IterItem, Option<T::LeafItem>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.next_and_focus() {
            Some((new, mess)) => {
                self.stack.push(mem::replace(&mut self.current, new));
                Some(Step::Forward(mess))
            },
            None => {
                let at_end = self.current.at_leaf();
                self.current = self.stack.pop()?;
                Some(Step::Backward(at_end))
            },
        }
    }
}
