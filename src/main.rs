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
    // Adding the ability to add a tag to each journal entry if desired. That way you can filter by tag later on.
    tag: Option<String>,
}

#[derive(Parser)]
struct Cli {
    // The first required arguement is the journal entry itself.
    #[arg(required = true, help = "The journal entry to be added")]
    entry: Vec<String>,

    // An optional arguement to allow the user to add a tag if they want to.
    #[arg(short, long, help = "Optional tag for this entry (e.g. bug, idea, note)")]
    tag: Option<String>,

    // An optional flag to display all unique tags in the journal.
    #[arg(long, help = "Display all unique tags in the journal")]
    show_tags: bool,

    // An arguement to read the journal entries.
    #[arg(long, help = "Used to read the journal entries")]
    read: bool,
}

fn main() {
    // First to set up the command line arguement parse so the user can call the program by just typing in "Wispword" followed by their entry.
    // If the Wispword command is called, anything that follows is treated as the journal entry.

    // Parse the command line arguments
    let args = Cli::parse();

    // If the user has specified the show_tags flag, we read the journal file and display all unique tags.
    if args.show_tags {
    let journal_path = Path::new("journal.json");

    // If the journal file doesn't exist, we inform the user and exit. This prevents errors or crashing.
    if !journal_path.exists() {
        eprintln!("No journal found.");
        std::process::exit(1);
    }

    // Read the journal file and deserialize the entries
    let content = fs::read_to_string(journal_path)
        .expect("Failed to read journal file");

    // Deserialize the journal entries, defaulting to an empty vector if parsing fails
    let entries: Vec<JournalEntry> = serde_json::from_str(&content)
        .unwrap_or_default();

    // Collect unique tags from the entries
    let mut tags = entries.iter()
        .filter_map(|e| e.tag.as_ref())
        .collect::<Vec<_>>();

    // Sort and deduplicate the tags
    tags.sort();
    tags.dedup();
    
    // Display the tags to the user
    if tags.is_empty() {
        println!("No tags found in journal.");
    } else {
        println!("Tags used in your journal:");
        for tag in tags {
            println!("- {}", tag);
        }
    }

    // Exit after displaying tags
    return; 
    }

    // If the user has specified the read flag, we read and display all journal entries.
    // God I need to refactor this. What am I doing. Why is everything in main. Brad, I'm sorry.
    if args.read {
    let journal_path = Path::new("journal.json");

    // Checks to see if the journal exists. If not, exit.
    // Later I will fix this to check earlier and create the journal if it doesn't exist.
    if !journal_path.exists() {
        eprintln!("No journal file found.");
        std::process::exit(1);
    }

    // Read the journal file and deserialize the entries
    let content = fs::read_to_string(journal_path).expect("Failed to read journal file");
    let entries: Vec<JournalEntry> = serde_json::from_str(&content).expect("Failed to parse journal");

    // If the journal is empty, inform the user. Otherwise, display all entries.
    if entries.is_empty() {
        println!("Your journal is empty.");
    } else {
        println!("\n📝 Your Journal Entries:\n");

        // For each entry, display the timestamp, tag (if any), and the entry text.
        // Does this actually help? Do we need the metadata or just the entry? Eh, we'll test and find out what works best.
        for (i, entry) in entries.iter().enumerate() {
            println!("Entry {}:", i + 1);
            println!("  Date: {}", entry.timestamp);
            println!("  Tag: {}", entry.tag.as_deref().unwrap_or("None"));
            println!("  Text: {}\n", entry.entry);
        }
    }

    return; // Exit after reading
}
    
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
        // Now also includes the optional tag in the journal entry.
        tag: args.tag,
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
