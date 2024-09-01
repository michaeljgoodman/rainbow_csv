use clap::Parser;
use colored::*;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

/// Simple CLI tool to print CSV files with rainbow-colored columns.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the CSV file
    file_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let colours = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];


    let file = File::open(cli.file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);


    for result in rdr.records() {
        let record = result?;

        for (i, field) in record.iter().enumerate() {
            let colour = colours[i % colours.len()];
            print!("{}", field.color(colour));
            if i < record.len() - 1 {
                print!("{}", ",".color(colour));
            }
        }
        println!();
    }

    Ok(())
}
