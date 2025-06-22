use std::error::Error;
use clap::{self, Parser, Subcommand};
use confy::{load, store};

use crate::logic::Sudoku;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    /// Commands to operate this game
    #[command(subcommand)]
    name: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new game
    New,
    /// Set a number(x, y, number) (1-9, 1-9, 1-9)
    Set { x: u32, y: u32, num: u32 },
    /// Print current Sudoku
    Print,
    /// Set difficulty(0-100), default is 50
    Diff { x: u32 },
}

pub fn shell() -> Result<(), Box<dyn Error>> {
    let mut cfg = load::<Sudoku>("Sudoku", "Sudoku")?;
    let cli = Cli::parse();
    match cli.name {
        Commands::New => cfg.flush(),
        Commands::Set { x, y, num } => {
            if cfg.check_idx(num, (x - 1) * 9 + y - 1) == true {
                if cfg.check_complete() {
                    println!("Completed!");
                } else {
                    println!("Right!");
                }
            } else {
                println!("Wrong!");
            }
            cfg.print_cur();
        }
        Commands::Print => cfg.print_cur(),
        Commands::Diff { x } => {
            if x <= 100 {
                cfg.set_diff((x as f64) / 100.0);
            } else {
                println!("Invalid difficulty, should be in 0-100.");
            }
        }
    }
    store("Sudoku", "Sudoku", cfg)?;
    Ok(())
}