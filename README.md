# FinderRunner

Use this to search for a program or a folder, get matching results and open what you searched for at once. 

## Final result

The app should by run by running `fir` and the search phrase and location. If no location is given it uses current location. There should also be some flags. `--fuzzy` Uses fuzzy search to find similar but misspelled results. `--regex` the search string is a regex string.

Examples:
```
fir -f hitman.exe C:\Games\

fir hitman.exe
```

I want a tui app where I can go up and down between results and click the one I want to open.

I want the tui interface to open at once, and the search to go immediately. When new results are found they should be added to the list in the tui. This might mean the app needs to be multi-threaded. One thread running the tui, and one, or more, searching the computer. 

Everytime you open a match, that folder gets added to saved to a list of high priority folders. These are saved and are the first ones to be searched.

## Things to do
Using the mvc pattern I need to implement several things. Listed in order of priority, kind of.


- Search
	- Goes through current folder and recursively goes through all files and folders within this folder. 
	- Should be as multithreaded as possible
	- Matched results get added to 

- View
	- Should be a TUI type app so we can immediately can choose our result, before having searched everything.
	- https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
	- One solution is to code the TUI very much by hand in crossterm.
	- Another solution is to do something like this https://monkeypatch.io/blog/2021/2021-05-31-rust-tui/