// story.rs

use crate::file_operations::append_to_file;
use std::{fs, io, path::PathBuf};

fn get_story_list_path() -> Result<PathBuf, String> {
    // Resolve the path to ~/effort-logger/story-list.txt
    let mut path: PathBuf = match dirs::home_dir() {
        Some(home) => home,
        None => return Err("Failed to determine home directory".to_string()),
    };
    path.push("effort-logger/story-list.txt");

    Ok(path)
}

pub fn read_story_list_from_file() -> Result<Vec<String>, String> {
    // Get the path to the story list file
    let path = match get_story_list_path() {
        Ok(path) => path,
        Err(e) => return Err(e), // Return the error if path resolution fails
    };
    // Read the file at the resolved path
    match fs::read_to_string(&path) {
        Ok(content) => {
            // Split the content by newlines and collect into a vector of strings
            let options = content.lines().map(|s| s.to_string()).collect();
            Ok(options)
        }
        Err(e) => Err(format!("Failed to read file at {:?}: {}", path, e)),
    }
}

pub fn add_story_to_list(story_id: String) -> Result<(), std::io::Error> {
    // Get the path to the story list file
    let path = match get_story_list_path() {
        Ok(path) => path,
        Err(e) => return Err(io::Error::new(io::ErrorKind::NotFound, e)), // Return error if path resolution fails
    };
    append_to_file(path, story_id)
}
