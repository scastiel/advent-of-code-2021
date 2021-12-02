use clap::{crate_name, crate_version, App, SubCommand};

extern crate clap;

mod day01;
mod day02;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("day-01"))
        .subcommand(SubCommand::with_name("day-02"))
        .get_matches();

    match matches.subcommand() {
        ("day-01", _) => day01::main(),
        ("day-02", _) => day02::main(),
        _ => (),
    }
}
