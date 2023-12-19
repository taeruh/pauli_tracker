# if you cannot run makefiles, you need to do the equivalent of the following:
# mkdir output
# # for the library
# cd pauli_tracker_clib
# cargo build --release
# cd ..
# cp pauli_tracker_clib/target/release/libpauli_tracker_clib.so output/ # for dynamic
# cp pauli_tracker_clib/target/release/libpauli_tracker_clib.a output/  # for static
# # for the header
# cd generate_bindings
# cargo run
# cd ..
# cp generate_bindings/output/pauli_tracker.h output/

output := output
lib = libpauli_tracker_clib
lib_path = pauli_tracker_clib/target/release/$(lib)
libs = $(lib_path).a $(lib_path).so
header = pauli_tracker.h
header_path = generate_bindings/output/$(header)


build: library header
library: $(output)/$(lib).so
header: $(output)/$(header)


$(output)/$(lib).so: $(lib_path).so | $(output)/
	cp $(libs) $(output)/

$(lib_path).so: FORCE
	(cd pauli_tracker_clib && cargo build --release)
	# (cd pauli_tracker_clib && CARGO_PROFILE_RELEASE_LTO=true \
	# 	RUSTFLAGS="-Clinker-plugin-lto -Clink-arg=-fuse-ld=lld" \
	# 	cargo +stable build --release)


$(output)/$(header): $(header_path) |  $(output)/
	cp $(header_path) $(output)/

$(header_path): FORCE
	(cd generate_bindings && cargo run --release)


%/:
	@mkdir -p $@

clean:
	rm -rf $(output)
	(cd pauli_tracker_clib && cargo clean)
	(cd generate_bindings && cargo clean && rm -rf output)


FORCE:

.PHONY: FORCE build
