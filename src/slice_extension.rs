/*!
Some additional slice methods.
*/

/// Helper trait to basically use something like [slice::get_many_mut], which is
/// currently unstable.
pub trait GetTwoMutSlice {
    type SliceType;

    unsafe fn get_two_unchecked_mut(
        &mut self,
        one: usize,
        two: usize,
    ) -> Option<(&mut Self::SliceType, &mut Self::SliceType)>;

    fn get_two_mut(
        &mut self,
        one: usize,
        two: usize,
    ) -> Option<(&mut Self::SliceType, &mut Self::SliceType)>;
}

// We are basically doing what std::slice does (cannot really use it because it is
// unstable at the moment), stripping down the chain of (unstable) method calls
impl<T> GetTwoMutSlice for [T] {
    type SliceType = T;

    /// # Safety
    ///
    /// The indices `one` and `two` have two different and in bounds.
    unsafe fn get_two_unchecked_mut(
        &mut self,
        one: usize,
        two: usize,
    ) -> Option<(&mut Self::SliceType, &mut Self::SliceType)> {
        let ptr: *mut T = self.as_mut_ptr();
        let a = unsafe { &mut *ptr.add(one) };
        let b = unsafe { &mut *ptr.add(two) };
        Some((a, b))
    }

    fn get_two_mut(
        &mut self,
        one: usize,
        two: usize,
    ) -> Option<(&mut Self::SliceType, &mut Self::SliceType)> {
        // we could have done that using std::slice::spli_at_mut, not needing to write
        // unsafe code our own here, but I feel like the unsafe code expresses better
        // what we are actually doing and it's invariants are pretty straightforward
        let len = self.len();
        if one == two || one > len || two > len {
            return None;
        }
        // Safety: the above conditational ensures that the requirements are fulfilled
        unsafe { self.get_two_unchecked_mut(one, two) }
    }
}
