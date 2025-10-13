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