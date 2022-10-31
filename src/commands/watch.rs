use clap::{arg, ArgMatches, Command};

pub struct Watch {}

impl Watch {
    pub fn subcmd() -> Command {
        Command::new("watch")
            .about("Watch for changes in your project and re-start the process")
            .arg(arg!([COMMAND] "The command to run on re-starts").required(true))
    }

    pub fn handle(_args: &ArgMatches) {
        println!("Used watch")
    }
}
