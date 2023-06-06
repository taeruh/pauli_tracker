use clap::{
    builder::EnumValueParser,
    Arg,
    ArgMatches,
    Command,
    ValueEnum,
};
use proc::ArgDispatch;

use crate::cicd;

#[derive(Clone, Copy, ValueEnum, ArgDispatch)]
#[arg_dispatch(module = "cicd")]
enum Test {
    Full,
    Hack,
    Msrv,
    Clippy,
    ClippyBeta,
    Docs,
    Fmt,
    Locked,
    Beta,
    OsCheck,
    Coverage,
    Proptest,
    Miri,
}

#[inline]
pub fn cli() -> Command {
    Command::new(crate::commands::command_name(file!()))
        .arg(
            Arg::new("test")
                .value_parser(EnumValueParser::<Test>::new())
                .num_args(1..)
                .value_name("TEST")
                .help("Run specified test"),
        )
        .arg_required_else_help(true)
}

pub fn run(args: &mut ArgMatches) {
    let tests = args.remove_many::<Test>("test").expect("tests cli should default");
    for test in tests {
        test.dispatch()
    }
}
