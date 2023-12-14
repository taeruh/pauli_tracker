use clap::{
    builder::EnumValueParser,
    Arg,
    ArgMatches,
    Command,
    ValueEnum,
};
use proc::ArgDispatch;

mod jobs;

#[derive(Clone, Copy, ValueEnum, ArgDispatch)]
#[arg_dispatch(module = "jobs")]
enum Job {
    Full,
    Hack,
    Msrv,
    Clippy,
    ClippyBeta,
    Docs,
    Fmt,
    Semver,
    PublicDeps,
    Standard,
    Beta,
    OsCheck,
    Coverage,
    Proptest,
    Miri,
}

pub fn cli() -> Command {
    Command::new(crate::commands::command_name(file!()))
        .arg(
            Arg::new("job")
                .value_parser(EnumValueParser::<Job>::new())
                .num_args(1..)
                .value_name("JOB")
                .help("Run specified test"),
        )
        .arg_required_else_help(true)
        .about("Run CI jobs")
}

pub fn run(args: &mut ArgMatches) {
    let tests = args.remove_many::<Job>("job").expect("tests cli should default");
    for test in tests {
        test.dispatch()
    }
}
