# rrl_cli_reader

rrl_cli_reader is a command-line interface (CLI) program for reading fiction from Royal Road. It allows you to load books, navigate through chapters, search for books and customize the text color. The program provides a simple and convenient way to enjoy Royal Road stories in a text-based format.

## Features

- Load books by entering the book ID.
- Search for books by title, author, tags, keywords, page amount, rating and status.
- View the list of chapters and choose a specific chapter.
- Sort search results by title, author, rating, views, pages, followers and last updated.
- Navigate through chapters with options to go to the previous or next chapter.
- Change the text color for a personalized reading experience.
- Easily quit the program or return to the main menu.

## Getting Started

### Build the Project

Make sure you have Rust installed. Clone the repository and use `cargo build --release` to build the executable. To build in GNU/Linux make sure libssl-dev and pkg-config are installed.

### Run the Program

Execute the built binary to launch the rrl_cli_reader.

### Navigate the Menu

Use the provided options (`P`, `B`, `C`, `L`, `Q`) to continue a previous book, load a new book, change text color, to search for a book, or quit the program.

### Read Chapters

Once a book is loaded, navigate through chapters using the options displayed. Press enter to progress through the story,  and type exit to bo back to the menu.

### Searching
First select what type of searching is going to be done. Then pick your ordering. Finally, type in your search query. The program will then display the results. If you want to read a book from the results, type in the number of the book you want to read. If you want to go back to the menu, type in exit.

## Usage when reading

- Press `<` to go to the previous chapter.
- Press `>` to go to the next chapter.
- Press `Q` to quit the program.
- Press `C` to view the list of chapters and choose a specific chapter.

## Dependencies

- colored: Terminal coloring for enhanced readability.
- crossterm: Cross-platform terminal manipulation library.
- serde: Serialization and deserialization framework.
- reqwest: HTTP client for making web requests.
- confy: Simple configuration management.

## Acknowledgments

Special thanks to the authors and contributors of the used Rust crates for making this project possible.

Enjoy your reading experience with rrl_cli_reader!
