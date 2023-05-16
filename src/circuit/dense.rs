// TODO finish implementation

//! [super::Circuit] is not memory optimal because [super::Gate] requires much padding
//! memory (because of the alignment). This can be circumvent by separating the gate
//! discriminator from the qubits the gate is acting on by keeping them in an separate
//! array. This module provides [Circuit] which does exactly that, however, not that
//! there are some API related drawbacks and there is a possible runtime cost.

/// A circuit description which can only be used as iterator. This limitation allows to
/// implement the circuit much more memory efficient than [super::Gate], however at
/// a runtime cost since iterating requires to perform a match on the gate (additionally
/// to a possible match operation in the user loop).
#[derive(Debug, Default)]
pub struct Circuit {
    gates: Vec<Gate>,
    gate_bits: Vec<usize>,
}

#[derive(Debug)]
/// A subset of the Clifford gates. They differ from [Gate] in that they do not store
/// the qubit position they act on.
pub enum Gate {
    X,
    // ...
    Cnot,
    Measure,
}

/// An iterator over [Circuit].
pub struct Iter<'c> {
    circuit: &'c Circuit,
    current_gate: usize,
    current_gate_bit: usize,
    len: usize,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            gates: Vec::new(),
            gate_bits: Vec::new(),
        }
    }

    pub fn x(&mut self, bit: usize) {
        self.gates.push(Gate::X);
        self.gate_bits.push(bit);
    }

    // ...

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            circuit: self,
            current_gate: 0,
            current_gate_bit: 0,
            len: self.gates.len(),
        }
    }

    // ...
}

impl Iterator for Iter<'_> {
    type Item = super::Gate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_gate < self.len {
            self.current_gate += 1;
            Some(match self.circuit.gates[self.current_gate] {
                Gate::X => {
                    self.current_gate_bit += 1;
                    super::Gate::X(self.circuit.gate_bits[self.current_gate_bit])
                }
                // ...
                Gate::Cnot => {
                    let next = self.current_gate + 1;
                    self.current_gate_bit += 2;
                    super::Gate::Cnot(
                        self.circuit.gate_bits[next],
                        self.circuit.gate_bits[self.current_gate_bit],
                    )
                }
                _ => todo!(),
            })
        } else {
            None
        }
    }
}

impl From<super::Circuit> for Circuit {
    fn from(value: super::Circuit) -> Self {
        let len = value.gates.len();
        let mut gates = Vec::with_capacity(len);
        let mut gate_bits = Vec::with_capacity(len); // at least len is required
        for gate in value.gates.iter() {
            match gate {
                super::Gate::X(bit) => {
                    gates.push(Gate::X);
                    gate_bits.push(*bit);
                }
                // ...
                super::Gate::Cnot(control, target) => {
                    gates.push(Gate::Cnot);
                    gate_bits.push(*control);
                    gate_bits.push(*target);
                }
                _ => todo!(),
            }
        }
        Circuit { gates, gate_bits }
    }
}
