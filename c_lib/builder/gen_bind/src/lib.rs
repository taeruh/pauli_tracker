#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
// opting out is the exception
#![warn(missing_copy_implementations)]

/*!
This library provides just a simple wrapper ([Generator]) around [cbindgen]'s
functionality (cf. [repo]) with sensible defaults for generating bindings based on the
[pauli_tracker] crate.

If you need more control, just use cbindgen directly, it's not complicated.

For an example, look into how we use it in [c_bindings] to generate basic C bindings.

[repo]: https://github.com/mozilla/cbindgen
[pauli_tracker]: https://github.com/taeruh/pauli_tracker
[c_bindings]:
git@github.com:QSI-BAQS/pauli_tracker_extern/blob/main/c_bindings/xtask/src/main.rs
*/

use std::{
    self, fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use cbindgen::{Builder, Config, Language, ParseConfig};

/// A simplified wrapper around [cbindgen]'s [Builder].
#[derive(Debug, Clone)]
pub struct Generator<T> {
    /// The name of the crate to generate bindings for.
    pub crate_name: String,
    /// The configuration passed to [Builder].
    pub config: GeneratorConfig,
    /// The actual workhorse.
    pub builder: Builder,
    state: PhantomData<T>,
}

/// First stage of the generator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uninitialized {}
/// Second stage of the generator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Initialized {}

/// The possible configuration options for the generator.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// The directory of the crate to generate bindings for, will default to ".".
    pub crate_dir: Option<PathBuf>,
    /// The directory to write the generated bindings to, will default to ".".
    pub output_dir: Option<PathBuf>,
    /// The name of the generated header file, will default to [Generator::crate_name].
    pub header_name: Option<String>,
    /// The suffix of the generated header file, will default to "h" for C, "hpp" for
    /// C++ and "pxd" for Cython.
    pub header_suffix: Option<String>,
    /// Whether to expand macros, will default to true. This requires that a nightly
    /// toolchain is installed. If you don't define any extern "C" functions through
    /// macros, you can set this to false.
    pub expand_macros: bool,
    /// A list of crate names to include in the bindings, will default to
    /// ["pauli_tracker"].
    pub includes: Option<Vec<String>>,
    /// The language to generate bindings for, will default to C.
    pub lang: Language,
    /// The original cbindgen configuration options. If this value is not `None`, it
    /// will overwrite all other options.
    pub cbindgen_config: Option<Config>,
}

impl Generator<Uninitialized> {
    /// Create a new generator to generate bindings for the crate with the given name.
    pub fn new<T: Into<String>>(crate_name: T) -> Self {
        Self {
            crate_name: crate_name.into(),
            config: GeneratorConfig::default(),
            builder: Builder::new(),
            state: PhantomData,
        }
    }

    /// Specify additional configuration.
    pub fn config(mut self, config: GeneratorConfig) -> Self {
        self.config = config;
        self
    }

    /// Combine [new](Self::new) and [config](Self::config).
    pub fn with_config<T: Into<String>>(crate_name: T, config: GeneratorConfig) -> Self {
        Self {
            crate_name: crate_name.into(),
            config,
            builder: Builder::new(),
            state: PhantomData,
        }
    }

    /// Do the initial setup for the generator.
    pub fn setup(self) -> Generator<Initialized> {
        if self.config.expand_macros {
            std::env::set_var("RUSTUP_TOOLCHAIN", "nightly");
        }

        let config = self.config.cbindgen_config.clone().unwrap_or_else(|| {
            let mut config: Config = Default::default();
            config.cpp_compat = true;
            config.language = Language::C;
            config.parse = ParseConfig {
                parse_deps: true,
                include: Some(
                    self.config
                        .includes
                        .clone()
                        .unwrap_or_else(|| vec!["pauli_tracker".into()]),
                ),
                expand: if self.config.expand_macros {
                    cbindgen::ParseExpandConfig {
                        crates: vec![self.crate_name.clone()],
                        ..Default::default()
                    }
                } else {
                    Default::default()
                },
                ..Default::default()
            };
            config
        });

        Generator {
            builder: self
                .builder
                .with_crate(match &self.config.crate_dir {
                    Some(path) => path.as_path(),
                    None => Path::new("."),
                })
                .with_config(config),
            crate_name: self.crate_name,
            config: self.config,
            state: PhantomData,
        }
    }
}

impl Generator<Initialized> {
    /// Specify the language to generate bindings for.
    pub fn set_lang(&mut self, language: Language) {
        self.config.lang = language;
    }

    /// Generate the bindings.
    pub fn generate(self) -> bool {
        if let Some(dir) = &self.config.output_dir {
            fs::create_dir_all(dir).expect("cannot create output directory");
        }

        self.builder
            .with_language(self.config.lang)
            .generate()
            .expect("unable to generate bindings")
            .write_to_file(
                self.config.output_dir.unwrap_or_else(|| PathBuf::from(".")).join(
                    format!(
                        "{}.{}",
                        self.config.header_name.unwrap_or(self.crate_name),
                        self.config.header_suffix.unwrap_or_else(|| {
                            match self.config.lang {
                                Language::C => "h",
                                Language::Cxx => "hpp",
                                Language::Cython => "pxd",
                            }
                            .into()
                        })
                    ),
                ),
            )
    }
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            crate_dir: None,
            output_dir: None,
            header_name: None,
            header_suffix: None,
            includes: None,
            expand_macros: true,
            lang: Language::C,
            cbindgen_config: None,
        }
    }
}

impl GeneratorConfig {
    /// Create a new default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `crate_dir` option.
    pub fn crate_dir<T: Into<PathBuf>>(mut self, crate_dir: T) -> Self {
        self.crate_dir = Some(crate_dir.into());
        self
    }

    /// Set the `output_dir` option.
    pub fn output_dir<T: Into<PathBuf>>(mut self, output_dir: T) -> Self {
        self.output_dir = Some(output_dir.into());
        self
    }

    /// Set the `header_suffix` option.
    pub fn header_name<T: Into<String>>(mut self, header_name: T) -> Self {
        self.header_name = Some(header_name.into());
        self
    }

    /// Set the `includes` option.
    pub fn includes<T: IntoIterator<Item = S>, S: Into<String>>(
        mut self,
        includes: T,
    ) -> Self {
        self.includes = Some(includes.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Specify whether to `expand_macros`.
    pub fn expand_macros(mut self, expand_macros: bool) -> Self {
        self.expand_macros = expand_macros;
        self
    }

    /// Specify the `language` to generate bindings for.
    pub fn lang(mut self, lang: Language) -> Self {
        self.lang = lang;
        self
    }

    /// Set the `cbindgen_config` option.
    pub fn cbindgen_config(mut self, cbindgen_config: Config) -> Self {
        self.cbindgen_config = Some(cbindgen_config);
        self
    }
}
