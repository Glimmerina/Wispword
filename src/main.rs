// V2 but refactored for better structure and error handling.
// Uses filesystem, path handling, clap for argument parsing and serde for serialisation/deserialisation
use std::fs;
use std::path::Path;
use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
// Setting up a config struct so we can load directory paths from a config file. Globalise the code!
struct Config {
    journal_path: String,
}

// Defines the structure of a journal entry with a timestamp, the actual content of the entry, and an optional tag.
#[derive(Serialize, Deserialize)]
struct JournalEntry {
    timestamp: String,
    entry: String,
    tag: Option<String>,
}

#[derive(Parser)]
struct Cli {
    // Allows arguements for the entry, a tag if desired, the option to show tags, read entries, and filter by tag.
    // Now also has an option to set the journal path and update the config file.
    #[arg(required = false, help = "The journal entry to be added")]
    entry: Vec<String>,

    #[arg(short, long, help = "Optional tag for this entry (e.g. bug, idea, note)")]
    tag: Option<String>,

    #[arg(long, help = "Display all unique tags in the journal")]
    show_tags: bool,

    #[arg(long, help = "Used to read the journal entries")]
    read: bool,

    #[arg(long, help = "Filter journal entries by a specific tag")]
    filter_tag: Option<String>,

    #[arg(long, help = "Set the path to the journal file and update config")]
    set_journal: Option<String>,

    #[arg(long, help = "Used to delete a journal entry by its index")]
    delete_entry: Option<usize>,
}

fn main() {

    // First to set up the command line argument parser so the user can call the program by just typing in "Wispword" followed by their entry.
    // If the Wispword command is called, anything that follows is treated as the journal entry.

    // Parse the command line arguments and get the journal path.
    let args = Cli::parse();
    let config = load_or_create_config();
    let journal_path = Path::new(&config.journal_path);

    // If the user requested to show tags, then show them.
    if args.show_tags {
        show_tags(journal_path);
        return;
    }

    // If the user wants to set a new journal path, update the config file and exit.
    if let Some(new_path) = &args.set_journal {
        let config_path = get_config_path();
        let config = Config {
            journal_path: new_path.clone(),
        };

    let serialized = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    fs::write(config_path, serialized).expect("Failed to write updated config");
    println!("Updated journal path to '{}'", new_path);
    return;
}

    // If the user prompted to read entries, then read them. If they gave a filter tag, apply it.
    if args.read {
        read_entries(journal_path, args.filter_tag);
        return;
    }

    // If no entry is provided, show an error and exit. Gives a help message too because I'm cool like that.
    if args.entry.is_empty() {
        eprintln!("No entry provided. Use --help for usage information.");
        std::process::exit(1);
    }

    // If the user wants to delete an entry, do so and exit.
    if let Some(entry_index) = args.delete_entry {
        delete_entry(journal_path, entry_index);
        return;
    }

    // Combine the entry arguments into a single string and add the entry to the journal.
    let combined_entry = args.entry.join(" ");
    if combined_entry.trim().is_empty() {
        eprintln!("Error: Journal entry cannot be empty.");
        std::process::exit(1);
    }

    // Add the new journal entry with optional tag.
    add_entry(journal_path, combined_entry, args.tag);
}

fn show_tags(journal_path: &Path) {
    // If the journal file doesn't exist, inform the user and exit.
    if !journal_path.exists() {
        eprintln!("No journal found.");
        std::process::exit(1);
    }

    // Read the journal file and deserialize the entries.
    let content = fs::read_to_string(journal_path).expect("Failed to read journal file");
    let entries: Vec<JournalEntry> = serde_json::from_str(&content).unwrap_or_default();

    // Collect unique tags from the entries.
    let mut tags: Vec<_> = entries.iter()
        .filter_map(|e| e.tag.as_ref())
        .collect();

    // Sort and deduplicate the tags.
    tags.sort();
    tags.dedup();

    // If the tag is has no entries, inform the user. Otherwise, display the tags.
    if tags.is_empty() {
        println!("No tags found in journal.");
    } else {
        println!("Tags used in your journal:");
        for tag in tags {
            println!("- {}", tag);
        }
    }
}

fn read_entries(journal_path: &Path, filter_tag: Option<String>) {
    // Check if the journal file exists. If not, exit.
    if !journal_path.exists() {
        eprintln!("No journal file found.");
        std::process::exit(1);
    }

    // Read the journal file and deserialize the entries.
    let content = fs::read_to_string(journal_path).expect("Failed to read journal file");
    let entries: Vec<JournalEntry> = serde_json::from_str(&content).expect("Failed to parse journal");

    // Apply filtering if a tag is provided.
    let filtered_entries: Vec<_> = match filter_tag {
        Some(ref tag) => entries
            .into_iter()
            .filter(|entry| entry.tag.as_deref() == Some(tag.as_str()))
            .collect(),
        None => entries,
    };

    // Display the entries to the user. If a filter was applied and no entries found, inform the user.
    if filtered_entries.is_empty() {
        match filter_tag {
            Some(tag) => println!("No entries found with tag '{}'.", tag),
            None => println!("Your journal is empty."),
        }
    } else {
        println!("\n📝 Your Journal Entries:\n");
        for (i, entry) in filtered_entries.iter().enumerate() {
            println!("Entry {}:", i + 1);
            println!("  Date: {}", entry.timestamp);
            println!("  Tag: {}", entry.tag.as_deref().unwrap_or("None"));
            println!("  Text: {}\n", entry.entry);
        }
    }
}

fn add_entry(journal_path: &Path, entry_text: String, tag: Option<String>) {
    // Ensure the journal file exists; if not, create it.
    if !journal_path.exists() {
        fs::File::create(journal_path).expect("Failed to create journal file");
    }

    // Create the new journal entry with the current timestamp, entry text, and optional tag.
    let new_entry = JournalEntry {
        timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        entry: entry_text,
        tag,
    };

    // Read existing entries, append the new entry, and write back to the file.
    let mut journal_entries: Vec<JournalEntry> = match fs::read_to_string(journal_path) {
        Ok(content) if !content.trim().is_empty() => {
            serde_json::from_str(&content).unwrap_or_else(|_| {
                eprintln!("Warning: Could not parse journal file.");
                Vec::new()
            })
        }
        _ => Vec::new(),
    };

    // Push the new entry and write back to the file.
    journal_entries.push(new_entry);

    // Serialize the updated entries and write them to the journal file.
    let serialized = serde_json::to_string_pretty(&journal_entries)
        .expect("Failed to serialize journal entries");

    // Write the serialized entries back to the journal file.
    fs::write(journal_path, serialized).expect("Failed to write to journal file");
    println!("Journal entry added successfully, darling!");
}

fn get_config_path() -> std::path::PathBuf {
    // Constructs the path to the config file in the user's Documents/Wispword directory.
    let mut path = dirs::document_dir().expect("Could not find Documents directory");
    // Ensure the Wispword directory is included in the path.
    path.push("Wispword");
    path.push("config.json");
    path
}

fn load_or_create_config() -> Config {
    // Loads the config file if it exists, otherwise creates a default config file.
    let config_path = get_config_path();

    // If the config file exists, read and parse it.
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .expect("Failed to read config file");
        serde_json::from_str(&content)
            .expect("Failed to parse config file")
    // If it doesn't exist, create a default config file.
    } else {
        // Create a default journal path in Documents/Wispword/journal.json
        let mut default_path = dirs::document_dir().expect("Could not find Documents directory");
        default_path.push("Wispword");
        default_path.push("journal.json");

        // Create the default config struct.
        let config = Config {
            journal_path: default_path.to_string_lossy().to_string(),
        };

        // Ensure the parent directory exists.
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create Wispword config directory");
        }

        // Serialize and write the default config to the config file.
        let serialized = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
        fs::write(&config_path, serialized).expect("Failed to write config file");

        config
    }
}

fn delete_entry(journal_path: &Path, entry_index: usize) {
    // Check if the journal file exists. If not, exit.
    if !journal_path.exists() {
        eprintln!("No journal file found.");
        std::process::exit(1);
    }

    // Read the journal file and deserialize the entries.
    let content = fs::read_to_string(journal_path).expect("Failed to read journal file");
    let mut entries: Vec<JournalEntry> = serde_json::from_str(&content).expect("Failed to parse journal");

    // Check if the entry index is valid.
    if entry_index == 0 || entry_index > entries.len() {
        eprintln!("Invalid entry index.");
        std::process::exit(1);
    }

    // Remove the specified entry.
    entries.remove(entry_index - 1);

    // Serialize the updated entries and write them back to the file.
    let serialized = serde_json::to_string_pretty(&entries).expect("Failed to serialize journal entries");
    fs::write(journal_path, serialized).expect("Failed to write to journal file");
    println!("Journal entry {} deleted successfully.", entry_index);
}