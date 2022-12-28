use std::io::{stdin, stdout, Write};
use nonogram_rs::*;

use clap::{Parser, Subcommand};
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{Color, Print, SetForegroundColor};

#[derive(Parser)]
#[command(version)]
#[command(about = include_str!("ABOUT"))]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Solve a nonogram and output the result
    Solve,
    /// Print a solved nonogram to the terminal
    Show
}

const BAD_LAYOUT: &str = "Bad layout or json.";
const BAD_NONOGRAM: &str = "Bad nonogram or json.";
const INVALID_LAYOUT: &str = "Invalid layout.";
const INVALID_COLOR: &str = "Nonogram contains invalid colors.";

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    match args.command {
        Command::Solve => solve(),
        Command::Show => show(),
    }
}

fn solve() -> Result<(), &'static str> {
    let layout: Layout<char> = serde_json::from_reader(stdin()).map_err(|_| BAD_LAYOUT)?;
    let nonogram = layout.solve(()).map_err(|_| INVALID_LAYOUT)?;

    serde_json::to_writer(stdout(),  &nonogram).unwrap();

    // Sometimes flush does not suffice.
    stdout().execute(Print("\n")).unwrap();

    Ok(())
}

fn show() -> Result<(), &'static str> {
    let nonogram: Nonogram<char> = serde_json::from_reader(stdin()).map_err(|_| BAD_NONOGRAM)?;

    for row in 0..nonogram.rows() {
        for col in 0..nonogram.cols() {
            match nonogram[(col, row)] {
                Cell::Box { color } => {
                    let c = map_color(color)?;

                    stdout().queue(SetForegroundColor(c)).unwrap();
                    stdout().queue(Print("██")).unwrap();
                }
                Cell::Space => {
                    stdout().queue(Print("  ")).unwrap();
                }
            }
        }
        stdout().queue(Print("\n")).unwrap();
    }
    stdout().flush().unwrap();

    Ok(())
}

fn map_color(color: char) -> Result<Color, &'static str> {
    match color {
        'd' => Ok(Color::Black),
        'r' => Ok(Color::DarkRed),
        'g' => Ok(Color::DarkGreen),
        'y' => Ok(Color::DarkYellow),
        'b' => Ok(Color::DarkBlue),
        'm' => Ok(Color::DarkMagenta),
        'c' => Ok(Color::DarkCyan),
        'w' => Ok(Color::Grey),
        'D' => Ok(Color::DarkGrey),
        'R' => Ok(Color::Red),
        'G' => Ok(Color::Green),
        'Y' => Ok(Color::Yellow),
        'B' => Ok(Color::Blue),
        'M' => Ok(Color::Magenta),
        'C' => Ok(Color::Cyan),
        'W' => Ok(Color::White),
        _ => Err(INVALID_COLOR)
    }
}