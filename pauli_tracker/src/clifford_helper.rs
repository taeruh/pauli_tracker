// requires that single_doc_standard!, coset!, double_doc! are in the local "use" scope
macro_rules! trait_gates {
    () => {
        // generators
        #[doc = single_doc_standard!("S")]
        fn s(&mut self, bit: usize);
        #[doc = single_doc_standard!("Hadamard")]
        fn h(&mut self, bit: usize);
        #[doc = double_doc!("Control Z")]
        fn cz(&mut self, bit_a: usize, bit_b: usize);

        #[doc = single_doc_standard!("I")]
        /// (The identity)
        fn id(&mut self, _: usize) {}
        coset!(id, "I", (x, "X"), (y, "Y"), (z, "Z"),);

        coset!(
            s,
            "S",
            (sdg, "S^dagger"),
            (sz, "sqrt(Z)"),
            (szdg, "sqrt(Z)^dagger"),
            (hxy, "H^{xy}"),
        );
        coset!(h, "H", (sy, "sqrt(Y)"), (sydg, "sqrt(Y)^dagger"),);

        #[doc = single_doc_standard!("SH")]
        fn sh(&mut self, bit: usize) {
            self.h(bit);
            self.s(bit);
        }

        #[doc = single_doc_standard!("HS")]
        fn hs(&mut self, bit: usize) {
            self.s(bit);
            self.h(bit);
        }

        #[doc = single_doc_standard!("SHS")]
        fn shs(&mut self, bit: usize) {
            self.s(bit);
            self.h(bit);
            self.s(bit);
        }
        coset!(
            shs,
            "SHS",
            (sx, "sqrt(X)"),
            (sxdg, "sqrt(X)^dagger"),
            (hyz, "H_yz"),
        );

        #[doc = double_doc!("Control X (Control Not)", control, target)]
        fn cx(&mut self, control: usize, target: usize) {
            self.h(target);
            self.cz(control, target);
            self.h(target);
        }

        #[doc = double_doc!("Control Y", control, target)]
        fn cy(&mut self, control: usize, target: usize) {
            self.hyz(target);
            self.cz(control, target);
            self.hyz(target);
        }

        #[doc = double_doc!("Swap")]
        fn swap(&mut self, bit_a: usize, bit_b: usize) {
            self.cx(bit_a, bit_b);
            self.cx(bit_b, bit_a);
            self.cx(bit_a, bit_b);
        }

        #[doc = double_doc!("iSwap")]
        fn iswap(&mut self, bit_a: usize, bit_b: usize) {
            self.s(bit_b);
            self.s(bit_a);
            self.h(bit_a);
            self.cx(bit_a, bit_b);
            self.cx(bit_b, bit_a);
            self.h(bit_b);
        }

        #[doc = double_doc!("iSwap^dagger")]
        /// Equivalent to the iSwap gate.
        fn iswapdg(&mut self, bit_a: usize, bit_b: usize) {
            self.iswap(bit_a, bit_b);
        }
    };
}
pub(crate) use trait_gates;
