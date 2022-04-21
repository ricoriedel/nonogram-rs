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

use std::io::stdin;
use clap::{Parser, ArgEnum};
use nonogram_rs::serde::{Layout, RawNonogram};

#[derive(Clone, PartialEq, ArgEnum)]
enum Format {
    Human,
    Json
}

#[derive(Parser)]
#[clap(
author = "Copyright (C) 2022 Rico Riedel <rico.riedel@protonmail.ch>",
version = env ! ("CARGO_PKG_VERSION"),
about = "License GPLv3+: GNU GPL version 3 or later <https://www.gnu.org/licenses/>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.")]
struct Args {
    #[clap(long, default_value = "human", help = "Set output format", arg_enum)]
    out_format: Format
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();
    let layout: Layout = serde_json::from_reader(stdin()).map_err(|_| "Invalid json.")?;
    let solution = layout.solve().map_err(|_| "Invalid nonogram.")?;

    print!("{}", match args.out_format {
        Format::Human => solution.to_string(),
        Format::Json => serde_json::to_string(&RawNonogram::from(solution)).unwrap(),
    });
    Ok(())
}