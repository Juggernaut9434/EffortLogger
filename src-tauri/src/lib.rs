// lib.rs

mod file_operations;

use file_operations::create_story_files;

pub fn setup() {
    // Make sure there is a story-id-list.txt file in 'effort-logger' folder
    // Run the create_story_list function on startup
    if let Err(e) = create_story_files() {
        eprintln!("Error creating story list: {}", e);
    }
}
