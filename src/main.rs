use clap::{crate_name, crate_version, App, SubCommand};

extern crate clap;

mod __template;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("__template"))
        .subcommand(SubCommand::with_name("day-01"))
        .subcommand(SubCommand::with_name("day-02"))
        .subcommand(SubCommand::with_name("day-03"))
        .subcommand(SubCommand::with_name("day-04"))
        .subcommand(SubCommand::with_name("day-05"))
        .subcommand(SubCommand::with_name("day-06"))
        .subcommand(SubCommand::with_name("day-07"))
        .get_matches();

    match matches.subcommand() {
        ("day-01", _) => day01::main(),
        ("day-02", _) => day02::main(),
        ("day-03", _) => day03::main(),
        ("day-04", _) => day04::main(),
        ("day-05", _) => day05::main(),
        ("day-06", _) => day06::main(),
        ("day-07", _) => day07::main(),
        ("__template", _) => __template::main(),
        _ => (),
    }
}
