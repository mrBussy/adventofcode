use adventofcode::challenges;
use adventofcode::Settings;
use clap::{App, Arg};

pub fn main() {
    let matches = App::new("Advent of Code")
                            .version("1.0")
                            .author("R. Middel <r.middel@mrbussy.eu>")
                            .about("Play with free time")
                            .arg(Arg::with_name("example")
                                .short("e")
                                .long("example")
                                .value_name("bool")
                                .help("run with examples")
                                .takes_value(false))
                            .arg(Arg::with_name("day")
                                .short("d")
                                .long("day")
                                .value_name("DAY")
                                .help("Set the day to run")
                                .takes_value(true))                            
                    .get_matches();

    // Set start with default settings
    let mut settings: Settings = Settings::new();
    settings.run_example = matches.is_present("example");

    match i8::from_str_radix(matches.value_of("day").unwrap_or("0").trim(), 10).unwrap_or(0) {
        1 => challenges::day1::run(settings),
        2 => challenges::day2::run(settings),
        3 => challenges::day3::run(settings),
        _ => challenges::run(settings),
    }

}