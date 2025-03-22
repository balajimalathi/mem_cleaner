// Copyright (c) 2025 <Your Name or Organization>
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use std::{collections::HashMap, fs, io, path::Path, path::PathBuf};
use dialoguer::{Confirm, Select, Input};

/// Function to get the directory to scan, either current or user-specified
fn get_directory() -> io::Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    
    // Ask user if they want to use the current directory
    let use_current = Confirm::new()
        .with_prompt(format!("Use current directory: {:?}?", current_dir))
        .default(true)
        .interact()
        .unwrap();

    if use_current {
        return Ok(current_dir);
    }

    // Loop until a valid directory path is provided
    loop {
        let input_path: String = Input::new()
            .with_prompt("Enter directory path")
            .interact_text()
            .unwrap();

        let path = PathBuf::from(&input_path);
        if path.exists() && path.is_dir() {
            return Ok(path);
        } else {
            println!("Invalid directory. Please enter a valid path.");
        }
    }
}

/// Function to scan the directory and find duplicate files based on name and size
fn scan_files(directory: &Path) -> io::Result<HashMap<(String, u64), Vec<PathBuf>>> {
    let mut file_map: HashMap<(String, u64), Vec<PathBuf>> = HashMap::new();
    
    // Iterate through all files in the directory and subdirectories
    for entry in walkdir::WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            if let Ok(metadata) = fs::metadata(&path) {
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let file_size = metadata.len();
                
                // Store files with the same name and size in a hashmap
                file_map.entry((file_name, file_size)).or_default().push(path);
            }
        }
    }
    Ok(file_map)
}

fn main() -> io::Result<()> {
    // Get the directory from the user
    let directory = get_directory()?;
    println!("Scanning directory: {:?}", directory);
    
    // Scan for duplicate files
    let file_map = scan_files(&directory)?;
    let mut unique_files = vec![];
    let mut duplicate_files = vec![];
    
    // Identify unique and duplicate files
    for (_key, paths) in file_map.iter() {
        if paths.len() > 1 {
            unique_files.push(paths[0].clone()); // Keep the first occurrence as unique
            duplicate_files.extend_from_slice(&paths[1..]); // Store the rest as duplicates
        }
    }
    
    // If no duplicates found, exit
    if duplicate_files.is_empty() {
        println!("No duplicate files found.");
        return Ok(())
    }
    
    println!("Found {} duplicate files.", duplicate_files.len());
    
    // Ask user how to handle duplicate files
    let choices = vec!["Delete All", "Move to Separate Folder", "Cancel"];
    let selection = Select::new()
        .with_prompt("Choose an action")
        .items(&choices)
        .default(2)
        .interact()
        .unwrap();
    
    match selection {
        0 => { // Delete all duplicate files
            if Confirm::new().with_prompt("Are you sure you want to delete all duplicate files?").interact().unwrap() {
                for file in &duplicate_files {
                    fs::remove_file(file)?;
                    println!("Deleted: {:?}", file);
                }
            }
        }
        1 => { // Move duplicate files to a separate folder
            let move_folder = directory.join("duplicates");
            fs::create_dir_all(&move_folder)?;
            for file in &duplicate_files {
                let destination = move_folder.join(file.file_name().unwrap());
                fs::rename(file, &destination)?;
                println!("Moved: {:?} -> {:?}", file, destination);
            }
        }
        _ => println!("Operation cancelled."),
    }
    Ok(())
}
