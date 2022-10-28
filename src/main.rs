mod utils {
    pub(crate) mod handler;
}
use clap::{arg, Command};
use utils::handler::{deploy, generate};

fn main() {
    let matches = Command::new("gy")
        .version("0.1")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .short_flag('g')
                .about("Scaffold a new project in given directory, Defaults to current directory")
                .arg(
                    arg!([PATH])
                    .default_value(".")
                ),
        )
        .subcommand(
            Command::new("deploy")
            .short_flag('d')
            .about(
            "Configure the settings for deploying your bot to one of the supported platforms",
        ).arg(
            arg!([PROVIDER])
            .required(true)
        ))
        .get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => generate(sub_matches.get_one::<String>("PATH")),

        Some(("deploy", sub_matches)) => deploy(sub_matches.get_one::<String>("PROVIDER")),

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
