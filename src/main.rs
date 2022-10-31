mod commands;
use clap::Command;
use commands::{deploy::Deploy, init::Init, watch::Watch};

fn main() {
    let matches = Command::new("gy")
        .version("0.1")
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(Deploy::subcmd())
        .subcommand(Init::subcmd())
        .subcommand(Watch::subcmd())
        .arg_required_else_help(true)
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => Init::handle(sub_matches),

        Some(("deploy", sub_matches)) => Deploy::handle(sub_matches),

        Some(("watch", sub_matches)) => Watch::handle(sub_matches),

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
