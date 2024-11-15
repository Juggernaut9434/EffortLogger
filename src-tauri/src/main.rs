// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_operations;

use std::fs;
use tauri::Emitter;
use file_operations::create_story_files;


#[tauri::command]
fn get_story_list() -> Result<Vec<String>, String> {
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
        },
        Err(e) => Err(format!("Failed to read file at {:?}: {}", path, e)),
    }
}

#[tauri::command]
fn navigate_to_page(window: tauri::Window, page_name: String) {
    // Emit an event to the frontend with the page name
    window.emit_to("main", "navigate", page_name).unwrap();
}

fn main() {
    /* SETUP */
    // Make sure there is a story-id-list.txt file in 'effort-logger' folder
    // Run the create_story_list function on startup
    if let Err(e) = create_story_files() {
        eprintln!("Error creating story list: {}", e);
    }

    println!("Starting Tauri application");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            navigate_to_page,  // Handler for navigating
            get_story_list        // Handler for loading story list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
