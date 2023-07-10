use std::{
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
    mem,
};

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

impl<T: FocusNext> Iterator for Sweep<T> {
    type Item = Step<T::Outcome, T::EndOutcome>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.step() {
            Some((new, mess)) => {
                unsafe { super::COUNT += 1 };
                self.stack.push(mem::replace(&mut self.current, new));
                Some(Step::Forward(mess))
            }
            None => {
                let at_end = self.current.check_end();
                self.current = self.stack.pop()?;
                Some(Step::Backward(at_end))
            }
        }
    }
}

pub trait FocusNext {
    type Outcome;
    type EndOutcome;
    fn step(&mut self) -> Option<(Self, Self::Outcome)>
    where
        Self: Sized;
    fn check_end(&self) -> Self::EndOutcome;
}

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
}
pub(crate) use impl_into_iterator;
