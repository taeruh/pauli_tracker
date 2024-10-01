use gen_bind::{Generator, GeneratorConfig};

fn main() {
    // cf. comments in pauli_tracker_lib/src/boolean_vector.rs
    std::env::set_var("RUSTFLAGS", "--cfg cbindgen");

    let different_header = Generator::with_config(
        "pauli_tracker_clib",
        GeneratorConfig::new()
            .crate_dir("../pauli_tracker_clib")
            .output_dir("output")
            .header_name("pauli_tracker"),
        // cf. comments in pauli_tracker_lib/src/boolean_vector.rs
        // .includes(["pauli_tracker", "bitvec"]),
    )
    .setup()
    .generate();

    // that's not always what I expected ...?
    if different_header {
        // println!("cbindgen: same header");
    }
}
