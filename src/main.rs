use std::fs;
use std::io::{self, stdin, Write};

fn main() {
    // Asks the user for a file path. Then flushes stdout to ensure the prompt is displayed immediately.
    print!("Enter the path to your file: ");
    io::stdout().flush().unwrap();

    // Reads the user input and trims any whitespace.
    let mut file_input = String::new();
    // If it fails to read the line, it will panic with an error message.
    stdin().read_line(&mut file_input).expect("Failed to read line");
    let path = file_input.trim();

    // Attempts to read the file at the specified path.
    // If successful, it prints "File found." If it fails, it prints an error message.
    match fs::read_to_string(path) {
        Ok(_) => println!("File found."),
        Err(e) => eprintln!("Error reading file '{}': {}", path, e),
    }

    //Now to let the user add to the file. It should append to the end of the file with a date and time stamp.
    print!("Add your entry, darling!: ");
    io::stdout().flush().unwrap();
    let mut journal_entry = String::new();
 
    stdin().read_line(&mut journal_entry).expect("Failed to read line");
    let text_to_append = journal_entry.trim();
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let entry = format!("[{}] {}\n", timestamp, text_to_append);
    match fs::OpenOptions::new().append(true).open(path) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", entry) {
                eprintln!("Error writing to file '{}': {}", path, e);
            } else {
                println!("Successfully appended to the file.");
            }
        }
        Err(e) => eprintln!("Error opening file '{}': {}", path, e),
    }


}
