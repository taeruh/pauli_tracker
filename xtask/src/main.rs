use xtask::commands;

macro_rules! run {
    ($($command:ident,)*) => {
        let (name, mut args) = commands::build()
            .get_matches()
            .remove_subcommand()
            .expect("we set arg_required_else_help(true) in commands::build");
        match name.as_str() {
            $(stringify!($command) => commands::$command::run(&mut args),)*
            other => {
                println!("The command \"{other}\" is not available yet.")
            }
        }
    };
}

fn main() {
    run!(ci, python_lib,);
}
