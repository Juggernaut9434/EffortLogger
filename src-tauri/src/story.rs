// story.rs

use std::fs;

pub fn read_story_list_from_file() -> Result<Vec<String>, String> {
    // Resolve the path to ~/effort-logger/story-list.txt
    let mut path = match dirs::home_dir() {
        Some(home) => home,
        None => return Err("Failed to determine home directory".to_string()),
    };
    path.push("effort-logger/story-list.txt");

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
