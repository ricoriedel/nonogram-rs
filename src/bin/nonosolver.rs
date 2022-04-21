//     A fast and lightweight library to solve nonograms.
//     Copyright (C) 2021  Rico Riedel <rico.riedel@protonmail.ch>
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fs;
use clap::{App, Arg, ArgMatches};
use nonogram_rs::serde::{Layout, RawNonogram};
use nonogram_rs::{Nonogram};

fn main() {
    let matches = App::new("NonoSolver")
        .version("1.0.1")
        .author("Copyright (C) 2021 Rico Riedel <rico.riedel@protonmail.ch>")
        .about(
r#"A fast and simple nonogram solver.
License GPLv3+: GNU GPL version 3 or later <https://www.gnu.org/licenses/>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
"#)
        .arg(Arg::new("in-file")
            .long("in-file")
            .required(false)
            .takes_value(true)
            .help("Input layout from file"))
        .arg(Arg::new("in-json")
            .long("in-json")
            .required(false)
            .takes_value(true)
            .help("Input layout from parameter"))
        .arg(Arg::new("out-file")
            .long("out-file")
            .required(false)
            .takes_value(true)
            .help("Write output to file"))
        .arg(Arg::new("out-format")
            .long("out-format")
            .help("Output format")
            .possible_values(["json", "human"])
            .default_value("human"))
        .get_matches();

    let json = get_json(&matches);
    let layout: Layout = serde_json::from_str(&json).expect("Json is malformed.");
    let nonogram = layout.solve().expect("The layout is invalid.");
    let output = get_output(&matches, nonogram);

    write_output(&matches, output);
}

fn get_json(matches: &ArgMatches) -> String {
    let in_file = matches.value_of("in-file")
        .map(|path| fs::read_to_string(path).expect("Failed to read file."));
    let in_json = matches.value_of("in-json")
        .map(|str| String::from(str));

    return in_file.or(in_json).expect("Either --in-json or --in-file must be provided.");
}

fn get_output(matches: &ArgMatches, nonogram: Nonogram) -> String {
    match matches.value_of("out-format").expect("--out-format is required.") {
        "human" => format!("{}", nonogram),
        "json" => serde_json::to_string(&RawNonogram::from(nonogram)).unwrap(),
        _ => panic!("Unknown format.")
    }
}

fn write_output(matches: &ArgMatches, data: String) {
    match matches.value_of("out-file") {
        Some(file) => fs::write(file, data).expect("Failed to write to file."),
        None => println!("{}", data)
    }
}