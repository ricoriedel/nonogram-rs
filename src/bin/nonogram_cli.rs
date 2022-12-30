use std::fmt::{Debug, Formatter};
use nonogram_rs::*;
use std::io::{stdin, stdout, Write};

use clap::{Parser, Subcommand};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::{ExecutableCommand, QueueableCommand};

#[derive(Parser)]
#[command(version)]
#[command(about = include_str!("ABOUT"))]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Solve a nonogram from stdin
    Solve,
    /// Print all found nonograms
    Show,
}

enum CliError {
    InvalidColor { color: char },
    ParsingError { error: serde_json::Error },
    IoError { error: std::io::Error },
}

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> Self {
        Self::ParsingError {
            error
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError {
            error
        }
    }
}

impl Debug for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InvalidColor { color } => write!(f, "invalid color: \"{}\"", color),
            CliError::ParsingError { error } => write!(f, "{}", error),
            CliError::IoError { error } => write!(f, "{}", error),
        }
    }
}

fn main() -> Result<(), CliError> {
    let args = Args::parse();

    match args.command {
        Command::Solve => solve(),
        Command::Show => show(),
    }
}

fn solve() -> Result<(), CliError> {
    let layout: Layout<char> = serde_json::from_reader(stdin())?;
    let collection = layout.solve(usize::MAX, ()).collection;

    serde_json::to_writer(stdout(), &collection)?;

    stdout().execute(Print("\n"))?;

    Ok(())
}

fn show() -> Result<(), CliError> {
    let collection: Vec<Nonogram<char>> = serde_json::from_reader(stdin())?;

    for nonogram in collection {
        print_nonogram(nonogram)?;
    }
    stdout().flush()?;

    Ok(())
}

fn print_nonogram(nonogram: Nonogram<char>) -> Result<(), CliError> {
    let width = nonogram.cols() * 2;
    let meta_width = width.saturating_sub(9);

    let cols = format!("Columns: {:>meta_width$}\n", nonogram.cols());
    let rows = format!("Rows:    {:>meta_width$}\n", nonogram.cols());

    stdout()
        .queue(Print("=".repeat(width)))?
        .queue(Print("\n"))?
        .queue(Print(cols))?
        .queue(Print(rows))?
        .queue(Print("-".repeat(width)))?
        .queue(Print("\n"))?;

    for row in 0..nonogram.rows() {
        for col in 0..nonogram.cols() {
            match nonogram[(col, row)] {
                Cell::Box { color } => {
                    let c = map_color(color)?;

                    stdout().queue(SetForegroundColor(c))?;
                    stdout().queue(Print("██"))?;
                }
                Cell::Space => {
                    stdout().queue(Print("  "))?;
                }
            }
        }
        stdout().queue(SetForegroundColor(Color::Reset))?;
        stdout().queue(Print("\n"))?;
    }
    Ok(())
}

fn map_color(color: char) -> Result<Color, CliError> {
    match color {
        '!' => Ok(Color::Reset),
        '0' => Ok(Color::Black),
        '1' => Ok(Color::DarkGrey),
        '2' => Ok(Color::Grey),
        '3' => Ok(Color::White),
        'R' => Ok(Color::Red),
        'G' => Ok(Color::Green),
        'Y' => Ok(Color::Yellow),
        'B' => Ok(Color::Blue),
        'M' => Ok(Color::Magenta),
        'C' => Ok(Color::Cyan),
        'r' => Ok(Color::DarkRed),
        'g' => Ok(Color::DarkGreen),
        'y' => Ok(Color::DarkYellow),
        'b' => Ok(Color::DarkBlue),
        'm' => Ok(Color::DarkMagenta),
        'c' => Ok(Color::DarkCyan),
        color => Err(CliError::InvalidColor { color }),
    }
}
