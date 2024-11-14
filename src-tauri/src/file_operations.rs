use std::fs::{File, create_dir_all, metadata};
use std::io::Write;
use dirs;

pub fn create_story_list() -> Result<(), String> {
    // Get the home directory path using dirs::home_dir
    let home_dir = dirs::home_dir().ok_or("Failed to find home directory")?;
    
    // Define the folder and file path
    let effort_logger_dir = home_dir.join("effort-logger");
    let story_file_path = effort_logger_dir.join("story-list.txt");

    // Create the "effort-logger" folder if it doesn't exist
    if !effort_logger_dir.exists() {
        create_dir_all(&effort_logger_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Check if the story-list.txt file already exists
    if metadata(&story_file_path).is_ok() {
        // If the file exists, do nothing and return
        return Ok(());
    }
    // If the file doesn't exist, create it. 
    else {
        // Create the story-list.txt file
        let mut file = File::create(story_file_path).map_err(|e| format!("Failed to create file: {}", e))?;
    
        // Optionally, write initial data to the file
        file.write_all(b"Story ID, Start DateTime, End DateTime, Duration\n")
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }

    Ok(())
}
