// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_operations;

use std::fs;
use tauri::Emitter;
use file_operations::create_story_list;


#[tauri::command]
fn get_story_list() -> Result<Vec<String>, String> {
    let path = "story-id-list.txt";
    match fs::read_to_string(path) {
        Ok(content) => {
            // Split the content by newlines and collect into a vector of strings
            let options = content.lines().map(|s| s.to_string()).collect();
            Ok(options)
        },
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

#[tauri::command]
fn navigate_to_page(window: tauri::Window, page_name: String) {
    // Emit an event to the frontend with the page name
    window.emit_to("main", "navigate", page_name).unwrap();
}

fn main() {
    /** SETUP **/
    // Make sure there is a story-id-list.txt file in 'effort-logger' folder
    // Run the create_story_list function on startup
    if let Err(e) = create_story_list() {
        eprintln!("Error creating story list: {}", e);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            navigate_to_page,  // Handler for navigating
            get_story_list        // Handler for loading story list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
