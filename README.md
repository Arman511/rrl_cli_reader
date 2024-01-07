# rrl_cli_reader

rrl_cli_reader is a command-line interface (CLI) program for reading fiction from Royal Road. It allows you to load books, navigate through chapters, and customize the text color. The program provides a simple and convenient way to enjoy Royal Road stories in a text-based format.

## Features

- Load books by entering the book ID.
- Navigate through chapters with options to go to the previous or next chapter.
- Change the text color for a personalized reading experience.
- Easily quit the program or return to the main menu.

## Getting Started

### Build the Project

Make sure you have Rust installed. Clone the repository and use `cargo build --release` to build the executable.

### Run the Program

Execute the built binary to launch the RRlCliReader.

### Navigate the Menu

Use the provided options (`P`, `B`, `C`, `Q`) to continue a previous book, load a new book, change text color, or quit the program.

### Read Chapters

Once a book is loaded, navigate through chapters using the options displayed. Press enter to progress through the story.

## Usage

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