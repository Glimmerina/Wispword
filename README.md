Wispword is a terminal based journaling program for making quick notes on the fly!

It allows you to write serialized entries into a .json journal whenever you need it! Each entry is date and timestamped for your convenience too!

Version 2:
Version 2 is now CLI based, no more having to execute the program every time! Simply type Wispword, followed by your journal entry, and it will add it to your journal! You can also add tags to entries if you need them (Eg Bug, Error, Warning, Success, or whatever is most applicable to what you're using it for!) by adding -- tagnamehere after your entry! It will also add the date and time, this happens automatically, no additional suffixes are required.

It will automatically create a Wispword folder in your documents that includes a config file. This tells Wispword where your journal file is located. You can update it with --Set-journal followed by the diectory!

List of commands (To be updated when this can be installed as a CLI tool)

Make an entry: Cargo run [Journal Text Goes here]
Make a multiple line entry: Cargo run -- --interactive (Type in one line. Press enter. Type in your next line. Continues until you press enter on a blank entry)
Make an entry with a tag: Cargo run [Journal Text Goes Here] --tag [Tag Name Goes Here]
Read your journal: Cargo run --read
Read your journal but filter by tag: Cargo run --read --filter-tag [Tag Name Goes Here]
Read journal but only show tags: Cargo run -- --show-tags
Change your journal location: Cargo run -- --set-journal [Directory Name here]
Generate journal statistics: Cargo run -- --stats

So far I think this will only work on Mac, but I can test it and find out!

Version 1:

Run Wispword to begin, it will ask for the location of your journal. If you have one, type its name! If not, don't worry, just type the name of what you would like to call your journal and Wispword will ask if you'd like it to make it for you! Also no need to worry about file extensions, Wispword automatically adds the .json extension for you!

After you've selected your journal, you will be prompted to write your entry, so write away, darling! 

When you've made an entry, Wispword will ask if you would like to make another. If you answer y/Y, it will repeat the journal entry process. If not, it will terminate.

I intend to make adjustments to allow it to be ran directly from the terminal in future without the need to execute the program each time, as well as let you have a specified journal directory if you need it.

But for now, this is my first solo project in Rust with no tutorials, Rustbook or guides! I hope you like it!
