//! This module handles only the parsing of the cli input and dispatching it to the
//! correct task. The main logic of the different "tasks" are not in this module.

pub mod ci;

use clap::{
    self,
    Command,
};

macro_rules! command {
    ($c:ident) => {
        crate::commands::$c::cli()
    };
}

pub fn build() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg_required_else_help(true)
        .subcommand(command!(ci))
        .infer_subcommands(true)
}

fn command_name(file: &str) -> &str {
    let (_, file) = file.rsplit_once('/').expect("we are at least in src/");
    file.strip_suffix(".rs").expect("it's a Rust file")
}
