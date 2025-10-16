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

fn main() {
    // Time for a fresh start!

}
