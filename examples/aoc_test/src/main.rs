use aoc_runner::*;
use clap::{App, Arg};

aoc_days!(aoc1, aoc2);

fn main() {
    let matches = App::new("Advent of Code 2021")
        .version("0.3")
        .author("Stefan Paun")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .help("Select the day")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap_or("0");

    match_day(day);
}
