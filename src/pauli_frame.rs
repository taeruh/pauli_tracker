// so we are currently using bitvec_simd as the bitvector implementation; bitvec_simd
// uses SIMD if possible; there's no stable safe way in std to use simd stuff, there's
// only a very low level module std::arch; bitvec_simd uses the crate wide which detects
// the target_feature(s) (at compile time) and wide itself uses the crate safe_arch
// which detects the target_arch(itectur) (at compile time); currently we are just
// trusting them to do things correctly; look into the build.rs where the
// target_feature(s) are enabled if available
use bitvec_simd::BitVec;

pub struct Frames {
    frames: Vec<BitVec>,
}
