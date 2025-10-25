Wispword is a terminal based journaling program for making quick notes on the fly!

It allows you to write serialized entries into a .json journal whenever you need it! Each entry is date and timestamped for your convenience too!

Version 2:
Version 2 is now CLI based, no more having to execute the program every time! Simply type Wispword, followed by your journal entry, and it will add it to your journal! You can also add tags to entries if you need them (Eg Bug, Error, Warning, Success, or whatever is most applicable to what you're using it for!) by adding -- tagnamehere after your entry! It will also add the date and time, this happens automatically, no additional suffixes are required.

You can now read your code by using the --read suffix! It can't filter individual tags or dates yet, but that's next!

Version 1:

Run Wispword to begin, it will ask for the location of your journal. If you have one, type its name! If not, don't worry, just type the name of what you would like to call your journal and Wispword will ask if you'd like it to make it for you! Also no need to worry about file extensions, Wispword automatically adds the .json extension for you!

After you've selected your journal, you will be prompted to write your entry, so write away, darling! 

When you've made an entry, Wispword will ask if you would like to make another. If you answer y/Y, it will repeat the journal entry process. If not, it will terminate.

I intend to make adjustments to allow it to be ran directly from the terminal in future without the need to execute the program each time, as well as let you have a specified journal directory if you need it.

But for now, this is my first solo project in Rust with no tutorials, Rustbook or guides! I hope you like it!
