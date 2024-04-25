use chrono::{DateTime, Utc};
use colored::*;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
// use std::fs::File;
// use std::io::Write;

fn main() {
    // Get working directory from argument
    let args: Vec<String> = std::env::args().collect();

    // Check if the user provided a directory path
    if args.len() < 2 {
        println!("Please provide a directory path");
        return;
    }

    // Check for help command
    if args[1] == "-h" || args[1] == "--help" {
        print_help();
        return;
    }

    // Get the working directory
    let path = &args[1];

    // Get the metadata of the directory
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => {
            println!("Directory does not exist");
            return;
        }
    };

    // Check if the path is a directory
    if metadata.is_dir() {
        // Get the total size of the directory
        let total_size = calculate_directory_size(path);

        // Get the number of files and directories in the directory
        let (file_count, directory_count) = get_number_of_files_directories(path);

        // Get the largest file in the directory
        let largest_file = get_largest_file(path);

        // Get most recently modified file with the date in (11/01/2024 10∶15∶53) format
        let most_recent_file = get_most_recently_modified_file(path);

        // Print the results
        print_metadata(
            &"Folder".to_string(),
            path,
            total_size,
            file_count,
            directory_count,
            largest_file,
            most_recent_file,
        );

        // // if "-txt" argument is provided, write the results to a text file
        // if args.len() > 2 && args[2] == "-txt" {
        //     let output = format!(
        //         "Metadata for: {}\r\nTotal folder size: {} bytes\r\nNumber of files: {}\r\nNumber of directories: {}\r\nLargest file: {} ({} bytes)\r\nMost recently modified file: {} ({}:UTC)\r\nDirectory tree:\r\n",
        //         path,
        //         total_size,
        //         file_count,
        //         directory_count,
        //         largest_file.as_ref().unwrap().file_name().to_str().unwrap(),
        //         largest_file.unwrap().metadata().unwrap().len(),
        //         most_recent_fcloneile.unwrap().0,
        //         most_recent_file.unwrap().1
        //     );
        //     let filename = format!("{}_metadata.txt", path);
        //     export_to_txt(&output, &filename).expect("Unable to write to file");
    } else {
        println!("{} is not a directory", path.blue());
        return;
    }
}

// Export the results to a text file (unused)
// fn export_to_txt(output: &str, filename: &str) -> std::io::Result<()> {
//     let mut file = File::create(filename)?;
//     file.write_all(output.as_bytes())?;
//     Ok(())
// }

fn print_help() {
    println!(
        "Usage: {} <{}>",
        "rust_file_system_analyzer".green(),
        "directory_path".yellow()
    );
    println!("\r\n{}", "Options:".bold());
    println!("  {} {}", "-h, --help".yellow(), "Print this help menu");
}

fn get_most_recently_modified_file(path: &str) -> Option<(String, String)> {
    let mut most_recent_file: Option<fs::DirEntry> = None;
    let mut most_recent_modified_time: Option<SystemTime> = None;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_metadata = entry.metadata().unwrap();
                let modified_time = entry_metadata.modified().unwrap();

                if most_recent_modified_time.is_none()
                    || modified_time > most_recent_modified_time.unwrap()
                {
                    most_recent_file = Some(entry);
                    most_recent_modified_time = Some(modified_time);
                }
            }
        }
    }

    if let Some(file) = most_recent_file {
        let modified_time = most_recent_modified_time.unwrap();
        let formatted_time = format_time(modified_time);
        Some((
            file.file_name().to_str().unwrap().to_string(),
            formatted_time,
        ))
    } else {
        None
    }
}

fn format_time(time: SystemTime) -> String {
    let formatted_time = time.duration_since(UNIX_EPOCH).unwrap();
    let formatted_time = formatted_time.as_secs();
    let formatted_time = DateTime::<Utc>::from_timestamp(formatted_time as i64, 0); // Convert to local time zone
    let formatted_time = formatted_time
        .map(|time| time.format("%m/%d/%Y %H:%M:%S").to_string())
        .unwrap_or(String::new());
    formatted_time
}

fn print_metadata(
    // file type (folder or file)
    filetype: &String,
    path: &String,
    total_size: u64,
    file_count: i32,
    directory_count: i32,
    largest_file: Option<fs::DirEntry>,
    most_recent_file: Option<(String, String)>,
) {
    println!("\r\n{} {}", "Metadata for:".bold(), path.blue());

    if filetype == "Folder" {
        println!(
            "Total folder size: {} kB",
            format!("{:.1}", total_size as f32 / 1000.0).yellow()
        );
        println!("Number of files: {}", file_count.to_string().green());
        println!(
            "Number of directories: {}",
            directory_count.to_string().green()
        );

        if let Some(largest_file) = largest_file {
            println!(
                "Largest file: {} ({} kB)",
                largest_file.file_name().to_str().unwrap().cyan(),
                format!(
                    "{:.1}",
                    largest_file.metadata().unwrap().len() as f32 / 1000.0
                )
                .to_string()
                .yellow()
            );
        }

        if let Some((file_name, modified_time)) = most_recent_file {
            println!(
                "Most recently modified file: {} ({}:UTC)",
                file_name.cyan(),
                modified_time.yellow()
            );
        }

        // Print fancy directory tree
        println!("\r\n{}", "Directory tree:".bold());
        print_directory_tree(path, 0);
    } else {
        println!(
            // "File size: {} bytes",
            // largest_file.metadata().unwrap().len().to_string().yellow()
            "Not supported yet"
        );
    }
    println!();
}

fn print_directory_tree(path: &String, depth: i32) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry_metadata = entry.metadata().unwrap();

        if entry_metadata.is_dir() {
            let entry_name = entry.file_name().to_str().unwrap().to_string();
            let entry_name = format!("{}{}{}", "│  ".repeat(depth as usize), "├─ ", entry_name);
            println!("{}", entry_name.green());
            print_directory_tree(&entry_path.to_str().unwrap().to_string(), depth + 1);
        } else {
            let entry_name = entry.file_name().to_str().unwrap().to_string();
            let entry_name = format!("{}{}{}", "│  ".repeat(depth as usize), "└─ ", entry_name);
            println!("{}", entry_name.cyan());
        }
    }
}

fn get_largest_file(path: &String) -> Option<fs::DirEntry> {
    let largest_file = fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .metadata()
                .ok()
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
        })
        .max_by_key(|entry| {
            entry
                .metadata()
                .ok()
                .map(|metadata| metadata.len())
                .unwrap_or(0)
        });
    largest_file
}

fn get_number_of_files_directories(path: &String) -> (i32, i32) {
    let mut file_count = 0;
    let mut directory_count = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_metadata = entry.metadata().unwrap();

                if entry_metadata.is_file() {
                    file_count += 1;
                } else if entry_metadata.is_dir() {
                    directory_count += 1;
                }
            }
        }
    }
    (file_count, directory_count)
}

fn calculate_directory_size(path: &str) -> u64 {
    let mut total_size = 0;

    // Iterate over the entries in the directory
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                let entry_metadata = entry.metadata().unwrap();

                if entry_metadata.is_file() {
                    // Add the size of the file to the total size
                    total_size += entry_metadata.len();
                } else if entry_metadata.is_dir() {
                    // Recursively calculate the size of the subdirectory
                    total_size += calculate_directory_size(entry_path.to_str().unwrap());
                }
            }
        }
    }

    total_size
}
