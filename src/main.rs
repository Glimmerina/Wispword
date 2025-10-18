//Uses filesystem, input/output, command line arguement, path handling, clap for arguement parsing and serde for serialization/deserialization
use std::fs;
use std::io::{self, stdin, Write};
use std::env::Args;
use std::path::Path;
use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
// Defines the structure of a journal entry with a timestamp and the actual content of the entry.
struct JournalEntry {
    timestamp: String,  
    entry: String,
}

#[derive(Parser)]
struct Cli {
    #[clap(required = true, help = "The journal entry to be added")]
    entry: String,
}

fn main() {
    // First to set up the command line arguement parse so the user can call the program by just typing in "Wispword" followed by their entry.
    // If the Wispword command is called, anything that follows is treated as the journal entry.

    // Parse the command line arguments
    let args = Cli::parse();

    if args.entry.trim().is_empty() {
        eprintln!("Error: Journal entry cannot be empty.");
        std::process::exit(1);
    }

    // Next we need to set up the journal file path and check if it exists, if not we create it.
    let journal_path = Path::new("journal.json");
    if !journal_path.exists() {
        fs::File::create(journal_path).expect("Failed to create journal file");
    }


}
