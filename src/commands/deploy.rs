use clap::{arg, ArgMatches, Command};

pub struct Deploy {}

impl Deploy {
    pub fn subcmd() -> Command {
        Command::new("deploy")
            .about(
                "Configure the settings for deploying your bot to one of the supported platforms",
            )
            .arg(arg!([PROVIDER]).required(true))
    }

    pub fn handle(args: &ArgMatches) {
        let parsed_args = match args.get_one::<String>("PROVIDER") {
            Some(a) => a,
            None => "",
        };
        println!("Used deploy {}", parsed_args)
    }
}
