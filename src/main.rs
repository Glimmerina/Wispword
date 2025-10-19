//Uses filesystem, path handling, clap for arguement parsing and serde for serialization/deserialization
use std::fs;
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
    #[arg(required = true)]
    entry: Vec<String>,
}

fn main() {
    // First to set up the command line arguement parse so the user can call the program by just typing in "Wispword" followed by their entry.
    // If the Wispword command is called, anything that follows is treated as the journal entry.

    // Parse the command line arguments
    let args = Cli::parse();

    // Combine the entry vector into a single string
    let combined_entry = args.entry.join(" ");
    if combined_entry.trim().is_empty() {
        eprintln!("Error: Journal entry cannot be empty, you silly thing!.");
        std::process::exit(1);
    }

    // Next we need to set up the journal file path and check if it exists, if not we create it.
    let journal_path = Path::new("journal.json");
    if !journal_path.exists() {
        fs::File::create(journal_path).expect("Failed to create journal file");
    }

    // Now to add a serialized entry to the journal file with a timestamp using serde and chrono
    let new_entry = JournalEntry {
        // Updated timestamp, no longer uses rfc3339 format, it's silly and not human friendly.
        timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        entry: combined_entry,
    };

    // Reads the journal file, reads the existing entries, appends the new entry, and writes it back to the file.
    // If the file is empty or cannot be parsed, it starts with an empty vector. Prevents crashing if the file is empty.
    let mut journal_entries: Vec<JournalEntry> = match fs::read_to_string(journal_path) {
        Ok(content) if !content.trim().is_empty() => {
            serde_json::from_str(&content).unwrap_or_else(|_| {
                eprintln!("Warning: Could not parse journal file.");
                Vec::new()
            })
        },
        _ => Vec::new(),
    };

    // Append the new entry and write back to the file
    journal_entries.push(new_entry);
    let serialized = serde_json::to_string_pretty(&journal_entries).expect("Failed to serialize journal entries");
    fs::write(journal_path, serialized).expect("Failed to write to journal file");
    println!("Journal entry added successfully, darling!");



}
