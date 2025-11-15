// V2 but refactored for better structure and error handling.
// Uses filesystem, path handling, clap for argument parsing and serde for serialisation/deserialisation
// Interactive Mode requires Self, Write.
// Hashmap is used for stats. We need it to count tags.
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    // Also has options to delete entries, create backups, enter interactively, and show stats. I'm on a roll with these features!
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

    #[arg(long, help = "Create a timestamped backup of the journal")]
    backup: bool,

    #[arg(long, help = "Enter journal entry interactively")]
    interactive: bool,

    #[arg(long, help = "Display journal usage statistics")]
    stats: bool,

    #[arg(long, help = "Search journal entries for a word or phrase")]
    search: Option<String>,
}


fn main() {

    // First to set up the command line argument parser so the user can call the program by just typing in "Wispword" followed by their entry.
    // If the Wispword command is called, anything that follows is treated as the journal entry.

    // Parse the command line arguments and get the journal path.
    let args = Cli::parse();
    let config = load_or_create_config();
    let journal_path = Path::new(&config.journal_path);

    // If the user wants to enter an entry interactively, prompt for it.
    if args.interactive {
        // If the user also provided entry text, show an error and exit.
        if !args.entry.is_empty() {
            eprintln!("Error: Do not use --interactive and entry text together.");
            std::process::exit(1);
        }
        // Prompt the user for the entry text.
        let entry = interactive_prompt();
        add_entry(journal_path, entry, args.tag);
        return;
    }

    // If the user requested to show tags, then show them.
    if args.show_tags {
        show_tags(journal_path);
        return;
    }

    // If the user wants to search for tags, search them and display all etries with that tag.
    if let Some(query) = args.search.clone() {
        search_entries(journal_path, &query);
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

    // How am I becoming an IF/ELSE developer. There has gotta be a better way than this.
    // If the user enters the stats command, show stats and exit.
    if args.stats {
        show_stats(journal_path);
        return;
    }

    // If the user wants to create a backup, do so and exit.
    if args.backup {
        create_backup(journal_path);
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

// If the user prompts to create a backup, this function will make one. Because I'm cool like that and data management is important.
fn create_backup(journal_path: &Path) {
    // Check if the journal file exists. If not, exit.
    if !journal_path.exists() {
        eprintln!("No journal file found to backup.");
        std::process::exit(1);
    }

    // Read the journal content
    let content = fs::read_to_string(journal_path)
        .expect("Failed to read journal file");

    // Generate a timestamped backup filename.

    let timestamp = chrono::Local::now().format("backup_%Y-%m-%d_%H-%M-%S.json");
    let backup_filename = timestamp.to_string();

    // Use the same directory as the journal file
    // I might update this later to create a Backups directory but for now, this will do.
    let backup_path = journal_path.with_file_name(backup_filename);

    // Write the backup file
    fs::write(&backup_path, content)
        .expect("Failed to write backup file");

    println!("Backup created: {}", backup_path.display());

}

// Interactive Mode: Lets the user type multi line entries and finish by entering an empty line.
fn interactive_prompt() -> String {
    // Prompts the user for a journal entry interactively.
    println!("🖋️ Wispword Interactive Mode\n");
    println!("Type your journal entry, darling! Press ENTER on an empty line to finish.");

    // Collect lines of the entry until an empty line is entered.
    let mut entry_lines = Vec::new();

    // Loop to read lines until an empty line is entered.
    loop {  
        // Prompt for input
        print!("> ");
        io::stdout().flush().unwrap();

        // Read a line from stdin
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        // Trim the line to remove trailing newline characters
        let trimmed = line.trim_end();

        // If the line is empty, break the loop. The entry is done.
        if trimmed.is_empty() {
            break;
        }

        // Otherwise, add the line to the entry.
        entry_lines.push(trimmed.to_string());
    }

    // Join the lines into a single entry string.
    let full_entry = entry_lines.join("\n");

    // If no entry was provided, exit with an error.
    if full_entry.trim().is_empty() {
        eprintln!("No entry entered. Aborting.");
        std::process::exit(1);
    }

    // Confirm entry capture.
    println!("\n✅ Entry captured!");
    full_entry
}

// Function to show journal statistics. I don't know why we'd ever need this but I'm running out of ideas for this app.
// It could be useful someday I guess.
fn show_stats(journal_path: &Path) {
    // Check if the journal file exists. If not, exit.
    if !journal_path.exists() {
        eprintln!("No journal file found.");
        std::process::exit(1);
    }

    // Read the journal file and deserialize the entries.
    let content = fs::read_to_string(journal_path).expect("Failed to read journal file");
    let entries: Vec<JournalEntry> = match serde_json::from_str(&content) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Failed to parse journal file.");
            std::process::exit(1);
        }
    };

    // If there are no entries, inform the user and exit.
    if entries.is_empty() {
        println!("Your journal is empty.");
        return;
    }

    // Variable for how many entries there are in total.
    let total = entries.len();

    // Variables for first and last entry timestamps.
    let first_entry = entries.first().unwrap().timestamp.clone();
    let last_entry = entries.last().unwrap().timestamp.clone();

    // Variables for the most common tags and how many entries don't have tags.
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    let mut untagged = 0;

    // For each entry in the journal, count the tags and those without tags.
    for entry in &entries {
        match &entry.tag {
            Some(tag) => *tag_counts.entry(tag.clone()).or_insert(0) += 1,
            None => untagged += 1,
        }
    }

    // Uses the hashmap to find the most common tag. If no tags exist, it shows "None".
    let most_common_tag = tag_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(tag, count)| format!("{} (used {} times)", tag, count))
        .unwrap_or_else(|| "None".to_string());

    // Calculates the average entry length in words. Averages are calculated to one decimal place.
    // I went with words rather than characters because anyone who counts their journaling by characters is a psycho.
    let total_words: usize = entries.iter()
        .map(|e| e.entry.split_whitespace().count())
        .sum();
    let average_words = total_words as f64 / total as f64;

    // Ends by displaying the stats. It's a bit of a wall of text but the alternative was to write them to a file.
    // All print their variables except for average words which is formatted to one decimal place.
    println!("\n📊 Wispword Journal Stats:\n");
    println!("- Total Entries: {}", total);
    println!("- First Entry: {}", first_entry);
    println!("- Most Recent Entry: {}", last_entry);
    println!("- Most Common Tag: {}", most_common_tag);
    println!("- Untagged Entries: {}", untagged);
    println!("- Average Entry Length: {:.1} words", average_words);
}

fn search_entries(journal_path: &Path, query: &str) {
    // If the journal file doesn't exist, inform the user and exit.
    if !journal_path.exists() {
        eprintln!("No journal found.");
        std::process::exit(1);
    }

    let content = fs::read_to_string(journal_path)
        .expect("Failed to read journal file");

    let entries: Vec<JournalEntry> = serde_json::from_str(&content)
        .unwrap_or_default();

    let query_lower = query.to_lowercase();

    let results: Vec<_> = entries.iter()
        .enumerate()
        .filter(|(_, entry)| {
            entry.entry.to_lowercase().contains(&query_lower)
                || entry.tag.as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&query_lower)
        })
        .collect();

    if results.is_empty() {
        println!("No entries found containing \"{}\".", query);
        return;
    }

    println!("🔍 Entries containing \"{}\":\n", query);

    for (i, entry) in results {
        println!("Entry {}:", i + 1);
        println!("  Date: {}", entry.timestamp);
        println!("  Tag: {}", entry.tag.as_deref().unwrap_or("None"));
        println!("  Text: {}\n", entry.entry);
    }
}