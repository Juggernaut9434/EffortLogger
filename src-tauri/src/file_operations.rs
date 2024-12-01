use dirs;
use std::fs::{create_dir_all, metadata, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

pub fn create_story_files() -> Result<(), String> {
    // Get the home directory path using dirs::home_dir
    let home_dir = dirs::home_dir().ok_or("Failed to find home directory")?;

    // Define the folder and file path
    let effort_logger_dir = home_dir.join("effort-logger");
    let story_file_path = effort_logger_dir.join("story-logs.txt");
    let story_list_path = effort_logger_dir.join("story-list.txt");

    // Create the "effort-logger" folder if it doesn't exist
    if !effort_logger_dir.exists() {
        create_dir_all(&effort_logger_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let _ = create_file(
        &story_file_path,
        b"Story ID, Start DateTime, End DateTime, Duration\n",
    );
    let _ = create_file(&story_list_path, b"Story ID\n");

    Ok(())
}

fn create_file(file_path: &PathBuf, initial_data: &[u8]) -> Result<(), String> {
    /* Create a file
     *
     * file_path: The path of the file to be created
     * initial_data: The initial data to be written to the file in binary format.
     */
    // Check if the story-list.txt file already exists
    if metadata(&file_path).is_ok() {
        // If the file exists, do nothing and return
        return Ok(());
    } else {
        // If the file doesn't exist, create it.
        // Create the story-list.txt file
        let mut file =
            File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;

        // Optionally, write initial data to the file
        file.write_all(initial_data)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    Ok(())
}

/// This function appends text to a file.
pub fn append_to_file(file_path: PathBuf, text: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Open in append mode
        .open(file_path)?; // Open the file (may return an error)

    writeln!(file, "{}", text)?; // Append the text and write a newline
    Ok(())
}
