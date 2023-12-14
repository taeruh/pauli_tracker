use std::{
    io,
    process::Command,
};

use clap::ArgMatches;

use crate::commands;

pub fn create_venv(_: &mut ArgMatches) {
    println!("creating the virtual environment (.venv) ...");
    commands::spawn(Command::new("python").args(["-m", "venv", ".venv"]));
}

pub fn pip_install(_: &mut ArgMatches) {
    if !check_venv() {
        return;
    }
    println!("installing the packages ...");
    commands::spawn(Command::new("pip").args(["install", "--upgrade", "pip"]));
    commands::spawn(Command::new("pip").args(["install", "-r", "requirements.txt"]));
}

pub fn pip_freeze(_: &mut ArgMatches) {
    if !check_venv() {
        return;
    }
    println!("first uninstalling 'pauli_tracker' (if installed) ...");
    commands::spawn(Command::new("pip").args(["uninstall", "pauli_tracker", "-y"]));
    println!("freeze!");
    let output = Command::new("pip")
        .arg("freeze")
        .output()
        .expect("failing to get the output");
    if !output.stderr.is_empty() {
        println!("error executing the command {:?}", output.stderr);
        return;
    }
    std::fs::write("requirements.txt", output.stdout)
        .expect("problem updating the requirements.txt file");
}

fn check_venv() -> bool {
    println!(
        "This commands must be run in the virtual envionment!\nIs the \
         environmentactivated? (yN):"
    );
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        println!("problem reading the input: {e:?}\n continuing with 'No'");
        0
    });
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    matches!(input.as_str(), "y" | "Y" | "yes" | "Yes" | "YES")
}
