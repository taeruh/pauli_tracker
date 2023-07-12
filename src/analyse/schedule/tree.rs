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

// should be able to error
pub trait Focus<Instruction> {
    type Error;
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

#[derive(Clone, Debug)]
pub enum Step<F, B> {
    Forward(F),
    Backward(B),
}

pub struct Sweep<T> {
    current: T,
    stack: Vec<T>,
}

#[derive(Debug)]
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

// well, now we are not really gaining anything from this macro, compared to
// implementing it separately -> TODO: make a derive proc-macro
// actually we are never calling it without the lifetime, but I let it here as it is,
// because it doesn't hurt
macro_rules! impl_into_iterator {
    ($name:ident) => {
        impl IntoIterator for $name {
            type Item = <Self::IntoIter as Iterator>::Item;
            type IntoIter = $crate::analyse::schedule::sweep::Sweep<Self>;
            fn into_iter(self) -> Self::IntoIter {
                Self::IntoIter::new(self, Vec::new())
            }
        }
    };
    ($name:ident with < $lifetime:tt >) => {
        impl<$lifetime> IntoIterator for $name<$lifetime> {
            type Item = <Self::IntoIter as Iterator>::Item;
            type IntoIter = $crate::analyse::schedule::tree::Sweep<Self>;
            fn into_iter(self) -> Self::IntoIter {
                Self::IntoIter::new(self, Vec::new())
            }
        }
    };
}
pub(crate) use impl_into_iterator;
