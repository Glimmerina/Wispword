The purpose of this project is to make an app that can make entries into a journal.

It should:

1) Add Entries with a date, tags and text.
2) Search entries by date or tag.
3) Export entries into a .json file (I've always wanted to learn how to use json files properly. Thank Skyrim modders for putting that curiosity into me)
4) Integrate with iTerm2. I want to be able to quickly write notes and entries from my terminal.

I briefly touched on reading/writing to files in the Rustbook but this is my own exeriment to learn how it all works for my own usage. Being able to quickly add notes and journal entries as I work with my own program would be pretty fun. 

I'll keep the development log updated from here:

Day 1:
Added the feature to locate a file and append to it. It will append your entry with a date and time stamp.

Tomorrow i want to add the ability to create a .json file if you don't already have one. 

Day 2:

Added the feature to create a .json file if you don't already have one. It will automatically prompt you to create your first entry too. Tomorrow I want to add a loop so you can make additional entries without re-running the program each time.

Day 3:

Added the loop. You can now add more entries if you wish. Next I want to add the ability to delete entries but I'm not sure the best way of doing that. Line by line perhaps? Delete entry 2, etc? I'll need to consider the options for this.

Day 4: 

Added serialization. Now it adds serialized entries, so it should be easier to select and delete entries.

Day 5:

Updated the code to only work with .json files. It has checks in place to ensure you always use .json files. You also don't have to type .json into your entry, it automatically does it for you!

Day 6:

The code has been refactored to include loops for repetition. The software is fully functional! You can search for files, create them and add entries to them at will! Huzzah!

Day 7:
The work on V2 has now begun. After fighting with github branches, I decided to instead rename the rust file to V1 and create a new main.rs that will serve as V2. I dislike branches on git, I don't ever want to merge into main, I want to be able to maintain two concurrent versions with different features and it seems like a new repository is the only "hygienic" way to do it? Bleh.

V2 will have command line functionality so you can call the code whenever you want to. Eg, in your terminal, "Wispword journal entry here", with Wispword acting as the keyword to execute the software and every word after being your journal entry. It will take a full rebuild I think, as I now need the env::args crate and a globalised config system. This is gonna be a challenge.