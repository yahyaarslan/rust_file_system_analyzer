# Rust File System Analyzer

This is a command-line application written in Rust that analyzes a given directory and provides useful metadata about it. 

## Features

- Calculates the total size of the directory.
- Counts the number of files and directories within the directory.
- Identifies the largest file in the directory.
- Identifies the most recently modified file in the directory.
- Prints a tree view of the directory structure.

## Usage

To use the application, pass the path of the directory you want to analyze as an argument:

```bash
cargo run /path/to/directory
```
This will print the metadata of the directory to the console.

## Options
- -h, --help: Print the help menu.

## Future Enhancements
- Support for analyzing individual files.
- Exporting the results to a text file.
