use clap::{arg, ArgMatches, Command};

pub struct Init {}

impl Init {
    pub fn subcmd() -> Command {
        Command::new("init")
            .about("Scaffold a new project in given directory, Defaults to current directory")
            .arg(arg!([PATH] "Path to create the project").default_value("."))
    }

    pub fn handle(_args: &ArgMatches) {}
}
