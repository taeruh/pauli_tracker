# C wrapper around pauli_tracker

C bindings partially exporting the API of the [pauli_tracker crate].

I would recommend to *not* use this library directly, but instead build your own C wrapper
for the specific functionality that you need. The `diy_example` provides a minimal example
to start. When doing this, it is still worth reading through the following sections,
because they can also be applied to your own library. The reason why I building you own
wrapper is because of the following: Firstly, this wrapper here is not well tested and not
that actively maintained. Secondly, while the provided API covers a lot of types and is
not really deep, and you may need to deal a lot with pointers when using it; it is just
more effective if you write your own wrapper with exactly the functionality that you need.

## Compiling the library

The header file is `pauli_tracker.h`. On Linux, run `make library` (do the equivalent of
what makefile does on window or macos) to generate a static and a dynamic library (`make
header` to rebuild the header). The files are located in `dist` (there pre-built
header file is in the current directory).

There's also a Dockerfile to build the library for Linux:
```bash
docker build --network=host -t pauli .
docker create --name pauli pauli:latest
docker cp pauli:/home/docker/dist/ .
```
*If you ran the commands as root, you may want to change the file permissions of the
copied files.*

Read the top level documentation of [pauli_tracker_clib]
(create it with `cargo doc --open` in the directory)!

**Note**: The generation of the header file requires that a nightly toolchain is
installed.

The provided API does not represent the full API of [pauli_tracker crate], it's only a
small subset of it. Some things that should probably be in it, may be randomly missing.
Feel free to open an issue, or better a PR, if you think something should provided. The
same is applies to the [imp_api] helper crate (see below).

If the provided functionality is not enough, or if the used naming convention for
explicit generics is too annoying, check out the next section.

The [example_usage] directory contains a basic example using the library from C++.
Enabling cross language LTO might be a little bit tricky (cf. [lto]); we don't do it per
default, compare the commented lines in the makefiles for the release builds).

## Building your own library

In the `diy_example` directory is a minimal example of how to build a compiled C library
with Rust for some Pauli tracker functionality.

The core Rust library [pauli_tracker crate] is highly generic; this feature is however
lost when creating a C API. Instead we have to implement more or less the same
boilerplate for every explicit type. This is where the two helper crates in [builder]
come in useful: [impl_api] can be used to automatically implement many of those
boilerplate things automatically via macros. The [gen_bind] crate (a wrapper around
[cbindgen]) can then be used to automatically generate the corresponding C header files
(it can also generate C++, and Cython headers, but they might need manual adjustments).
Check out their documentation (run `cargo doc --open` in the correspinding directories)
and how we use them in [pauli_tracker_clib] (the wrapper around [pauli_tracker crate])
and [generate_bindings].

You can use this to build your own library similar to how we do it. You don't have to
copy what is already provided; just build an additional library and link to both. In
this case the comment `/// cbindgen:ignore` might be handy, to prevent duplicate type
definitions.

Same as above, feel free to open issues, or better PRs, regarding additional
functionality for the [imp_api] crate.

*Note that [gen_bind] is effectively just a simple, restricted wrapper around [cbindgen]
to help getting started. If things get a little bit more complicated one should use
cbindgen directly (it's really simple) and gen_bind's cbindgen configuration maybe as
initial template. Also note, that while cbindgen does a pretty good job for most
use cases, one might to customize the output in certain cases.*

### Notes about cbindgen

- Don't rename dependencies and don't name the wrapping library like a dependency. This
  makes problems is the resolution of types.
- Default generic parameters are problematic, just specify them.

## Some notes about linking

If you are linking the library as static archive with some older linker version, you might
need to manually link to some native libraries, e.g., `-lgcc_s -lutil -lrt -lpthread
-lm -ldl -lc`. You can check which libraries are needed with `cargo rustc -- --print
native-static-libs`. This is probably not necessary for newer linker versions.

## Cross language LTO

Building the library, and using it, with cross language LTO enabled is a little bit
tricky. There's some information on Rust's [linker-plugin-lto] website. In short: To
make this work, the LLVM version's used by the `rustc` compiler, the linker invoked by
`rustc`, `clang` and the linker invoked by `clang` have to be the same version, and the
linker must be able to handle lto . You can view `rustc`'s LLVM version with `rustc -vV`
and similar for `clang` and the `lld` linker. If you are lucky, that's the same version
for the LLVM tools that your OS provides. If not, you probably have to install install
them from source. To do that, follow the instructions on the LLVM website, e.g., for
the [lld-linker] you can do something like the following (similar for the `clang`
project):
```bash
git clone https://github.com/llvm/llvm-project
cd llvm-project
git checkout origin/release/<the_required_version>
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_PROJECTS=lld \
  -DCMAKE_INSTALL_PREFIX=<where_you_want_to_install_it> ../llvm
make install
```
*The install command may take some time.., you may want to configure it if you don't
want to install everything.* Depending on the your OS, the following has
to be in the path:`ld.lld (Unix), ld64.lld (macOS), lld-link (Windows), wasm-ld
(WebAssembly)` (you might also want to put `lld`, which should link to the appropriate
version into your path).

With `clang` and `lld` installed, you can compile a Rust
library, e.g., a `staticlib`, with
```bash
CARGO_PROFILE_RELEASE_LTO=true \
  RUSTFLAGS="-Clinker-plugin-lto -Clink-arg=-fuse-ld=lld" \
  cargo build --release
```
or similar configuration in a `.cargo/config.toml`. Setting the linker here to `lld`
should be only necessary, I think, if the Rust crate has dependencies (which
`pauli_tracker_clib` does). When that worked, you should be able to link to it
with something like
```bash
clang -flto= -fuse-ld=path/to/lld -L path/to/rust/lib -l"rust-lib" <other_stuff>
```
Also, compare the commented lines in the makefiles for the release builds.

[builder]: ./builder
[cbindgen]: https://github.com/mozilla/cbindgen
[example_usage]: ./example_usage
[generate_bindings]: ./generate_bindings
[gen_bind]: ./builder/gen_bind
[imp_api]: ./builder/impl_api/
[linker-plugin-lto]: https://doc.rust-lang.org/rustc/linker-plugin-lto.html
[lld-linker]: https://lld.llvm.org/
[lto]: #cross-language-lto
[pauli_tracker_clib]: ./pauli_tracker_clib/
[pauli_tracker crate]: ../pauli_tracker/
