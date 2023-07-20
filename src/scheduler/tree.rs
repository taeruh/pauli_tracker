/*!
...
*/

use std::{
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
    mem,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

pub trait Focus<Instruction> {
    type Error;

    fn focus_inplace(&mut self, instruction: Instruction) -> Result<(), Self::Error>;

    // often just {let mut new = self.clone(); new.focus_inplace(instruction)?; Ok(new)}
    // but we don't want the Clone bound here
    fn focus(&mut self, instruction: Instruction) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait FocusIterator {
    type IterItem;
    type LeafItem;

    fn next_and_focus(&mut self) -> Option<(Self, Self::IterItem)>
    where
        Self: Sized;

    fn at_leaf(&self) -> Option<Self::LeafItem>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Step<F, B> {
    Forward(F),
    Backward(B),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sweep<T> {
    current: T,
    stack: Vec<T>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmptyStackError;
impl Display for EmptyStackError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "the stack is empty")
    }
}
impl Error for EmptyStackError {}

impl<T> Sweep<T> {
    pub fn new(current: T, stack: Vec<T>) -> Self {
        Self { current, stack }
    }

    pub fn current(&self) -> &T {
        &self.current
    }

    pub fn stack(&self) -> &Vec<T> {
        &self.stack
    }

    pub fn skip_focus(&mut self) -> Result<(), EmptyStackError> {
        self.current = self.stack.pop().ok_or(EmptyStackError)?;
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
            }
            None => {
                let at_end = self.current.at_leaf();
                self.current = self.stack.pop()?;
                Some(Step::Backward(at_end))
            }
        }
    }
}
