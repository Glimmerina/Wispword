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
        // If it finds the file, all is well.
        Ok(_) => println!("File found."),
        Err(e) => {
            // If it doesn't find the file, it will ask if you want to create it.
            eprintln!("Could not find file at '{}': {}", path, e);
            print!("Would you like to create it? (y/n): ");
            io::stdout().flush().unwrap();

            let mut create_input = String::new();
            stdin().read_line(&mut create_input).expect("Failed to read line");

            // If the user inputs 'y' or 'Y', it will attempt to create the file.
            // If it fails to create the file, it will print an error message.
            // If the user inputs anything else, it will exit the program.
            if create_input.trim().eq_ignore_ascii_case("y") {
                match fs::File::create(path) {
                    Ok(_) => println!("File created at '{}'.", path),
                    Err(e) => eprintln!("Failed to create file at '{}': {}", path, e),
                }
            } else {
                println!("Exiting program.");
                return;
            }
        }
    }

    loop {
        //Now to let the user add to the file. It should append to the end of the file with a date and time stamp.
        print!("Add your entry, darling!: ");
        io::stdout().flush().unwrap();
        let mut journal_entry = String::new();
        
        // Reads the user input for the journal entry. If it fails, it will panic with an error message.
        stdin().read_line(&mut journal_entry).expect("Failed to read line");
        // Appends the entry to the end with a timestamp.

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
        // Asks if the user would like to add another entry
        print!("Would you like to add another entry? (y/n)");
        io::stdout().flush().unwrap();

        let mut repeat_input = String::new();
        stdin().read_line(&mut repeat_input).expect("Failed to read line");
        // If they say yes, repeat. If no, break the loop.
            if repeat_input.trim().eq_ignore_ascii_case("y") {

            } else {
                print!("Very well, farewell, darling!");
                break;
            }
}   


}
