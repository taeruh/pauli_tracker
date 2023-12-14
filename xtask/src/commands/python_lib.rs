use clap::{
    ArgMatches,
    Command,
};

mod tasks;

pub fn cli() -> Command {
    Command::new(crate::commands::command_name(file!()))
        .subcommand(Command::new("create_venv").about("Create the virtual environment"))
        .subcommand(
            Command::new("pip_install")
                .about("Install the packages in the requirements.txt file"),
        )
        .subcommand(
            Command::new("pip_freeze")
                .about("Freeze the pip packages into the requirements.txt file"),
        )
        .subcommand_required(true)
        .about("Commands to setup stuff in the python_lib wrapper")
}

macro_rules! run {
    ($args:expr, $($command:ident,)*) => {
        let (name, mut args) = $args
            .remove_subcommand()
            .expect("we set subcommand_required(true) in cli");
        match name.as_str() {
            $(stringify!($command) => tasks::$command(&mut args),)*
            other => {
                println!("The command \"{other}\" is not available yet.")
            }
        }
    };
}

pub fn run(args: &mut ArgMatches) {
    run!(args, create_venv, pip_install, pip_freeze,);
}
