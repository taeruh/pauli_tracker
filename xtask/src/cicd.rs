use std::{
    env,
    fs,
    io::{
        Read,
        Seek,
        SeekFrom,
        Write,
    },
    path::PathBuf,
    process::Command,
};

macro_rules! cargo {
    ($($arg:literal),*) => {
        crate::cicd::_cargo([$($arg),*])
    };
    ($(($key:literal,$value:literal),)+ $($arg:literal),*) => {
        crate::cicd::_cargo_envs([$($arg),*], [$(($key, $value)),*])
    };
}

// {{ check
pub fn hack() {
    println!("CHECK: HACK");
    cargo!("hack", "check", "--feature-powerset", "--no-dev-deps")
}

pub fn msrv() {
    println!("CHECK: MSRV");
    println!(
        "skipped (don't expect contributors to have multiple Rust versions installed)"
    );
}

pub fn clippy() {
    println!("CHECK: CLIPPY");
    cargo!(
        ("RUSTFLAGS", "-Dwarnings"),
        "clippy",
        "--all-targets",
        "--all-features"
    );
}

pub fn clippy_beta() {
    println!("CHECK: CLIPPY BETA");
    cargo!(
        ("RUSTFLAGS", "-Dwarnings"),
        "+beta",
        "clippy",
        "--all-targets",
        "--all-features"
    );
}

pub fn docs() {
    println!("CHECK: DOCS");
    cargo!(
        ("RUSTDOCFLAGS", "-Dwarnings --cfg docsrs"),
        "+nightly",
        "doc",
        "--no-deps",
        "--all-features"
    );
}

pub fn fmt() {
    println!("CHECK: FMT");
    // cargo fmt ignores workspace default-members
    cargo!("+nightly", "fmt", "--check", "--package", "pauli_tracker");
}

pub fn semver() {
    println!("CHECK: SEMVER");
    cargo!("semver-checks", "--package", "pauli_tracker");
}

pub fn public_deps() {
    fn r#try() -> anyhow::Result<()> {
        let dir: String = env::current_dir()?
            .into_os_string()
            .into_string()
            .expect("problem reading OsString");
        println!("{:?}", dir);
        let idx = match dir.find("pauli_tracker") {
            Some(idx) => idx,
            None => {
                println!("Execute this command in the repository.");
                return Ok(());
            }
        };
        let (repo_dir, _) = dir.split_at(idx);
        let mut manifest_file = PathBuf::from(repo_dir);
        manifest_file.push("pauli_tracker/pauli_tracker/Cargo_nightly.toml");
        let manifest_nightly = fs::read(&manifest_file)?;
        manifest_file.pop();
        manifest_file.push("Cargo.toml");
        let mut manifest =
            fs::File::options().read(true).write(true).open(manifest_file)?;
        let size = manifest.metadata().map(|m| m.len() as usize).ok();
        let mut manifest_standard = Vec::with_capacity(size.unwrap_or(0));
        manifest.read_to_end(&mut manifest_standard)?;
        manifest.set_len(0)?;
        manifest.seek(SeekFrom::Start(0))?;
        manifest.write_all(&manifest_nightly)?;
        cargo!(
            ("RUSTFLAGS", "-Dwarnings"),
            "+nightly",
            "check",
            "--package",
            "pauli_tracker",
            "--all-features"
        );
        manifest.set_len(0)?;
        manifest.seek(SeekFrom::Start(0))?;
        manifest.write_all(&manifest_standard)?;
        Ok(())
    }
    r#try().expect("problem checking for public dependencies")
}
// }}

// {{ test
pub fn standard() {
    println!("TEST: STANDARD");
    cargo!("+nightly", "update", "-Zdirect-minimal-versions");
    cargo!("test", "--locked", "--all-features", "--all-targets");
    cargo!("test", "--locked", "--all-features", "--doc");
}

pub fn beta() {
    println!("TEST: BETA");
    cargo!("+beta", "test", "--all-features", "--all-targets");
    cargo!("+beta", "test", "--all-features", "--doc");
}

pub fn os_check() {
    println!("TEST: OS_CHECK");
    println!("skipped (obviously)");
}

pub fn coverage() {
    println!("TEST: COVERAGE");
    cargo!("+nightly", "llvm-cov", "--all-features", "--doctests");
}

pub fn proptest() {
    println!("TEST: PROPTEST");
    cargo!("+nightly", "update", "-Zdirect-minimal-versions");
    cargo!(
        "test",
        "--locked",
        "--release",
        "proptest",
        "--all-features",
        "--",
        "--ignored"
    );
}
// }}

// {{ safety
pub fn miri() {
    println!("SAFETY: MIRI");
    // the following problem cannot happen anymore, because xtask is not in the
    // workspace anymore, but it's still good to know
    //
    // something weird is happening here: when we do `cargo run` for this package,
    // miri starts to execute but fails with the error that we are not on nightly,
    // which is not true; we don't get this error if we directly execute the binary
    // manually (after building it); a wild guess what the problem could be: when we
    // do `cargo run` cargo sets a certain (environment) variable, let's call it V,
    // (don't know which; tried a few things) to "stable", then, when this command
    // here is executed, cargo sees the +nightly, so it starts miri, however, it
    // does not set V to "nightly" and miri reads V so it thinks we are not on
    // nightly; note that we do not have that problem in the [check::docs] function
    cargo!("+nightly", "miri", "test", "--all-features", "--lib", "--tests");
    cargo!("+nightly", "miri", "test", "--all-features", "--doc");
}
// }}

pub fn full() {
    // sorted so that we don't have to recompile that often (with limited success ...)
    msrv();
    os_check();
    docs();
    fmt();
    semver();
    public_deps();
    miri();
    beta();
    clippy_beta();
    clippy();
    standard();
    proptest();
    hack();
    coverage();
}

fn _cargo<const N: usize>(args: [&str; N]) {
    spawn(Command::new("cargo").args(args));
}

fn _cargo_envs<const N: usize, const M: usize>(
    args: [&str; N],
    envs: [(&str, &str); M],
) {
    spawn(Command::new("cargo").args(args).envs(envs));
}

fn spawn(command: &mut Command) {
    command
        .spawn()
        .expect("failed to spawn cargo command")
        .wait()
        .expect("failed to wait for cargo command");
}
