use bit_vec::BitVec;

#[derive(Debug, Default)]
pub struct Frames {
    // note that we are effectively using an array of array; this wouldn't be optimal if
    // the inner array has a fixed size (then one could do the usual thing and flatten
    // the arrays into one array), however, this is not necessarily true for us since we
    // might continuesly add frames and remove qubits (when it is measured) to reduce
    // the required memory
    frames: Vec<BitVec>,
}

impl Frames {
    pub fn new() -> Self {
        Frames { frames: vec![BitVec::new()] }
    }
}
