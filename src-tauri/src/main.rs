// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_operations;
mod story;

// #[cfg(feature = "cli")]
mod cli;

// #[cfg(feature = "cli")]
use cli::run_cli;

use effort_logger_lib::setup;
use story::read_story_list_from_file;
use tauri::Emitter;

#[tauri::command]
fn get_story_list() -> Result<Vec<String>, String> {
    read_story_list_from_file()
}

#[tauri::command]
fn navigate_to_page(window: tauri::Window, page_name: String) {
    // Emit an event to the frontend with the page name
    window.emit_to("main", "navigate", page_name).unwrap();
}

#[cfg(not(feature = "cli"))]
fn main() {
    setup();

    println!("Starting Tauri application");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            navigate_to_page, // Handler for navigating
            get_story_list    // Handler for loading story list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(feature = "cli")]
fn main() {
    run_cli();
}
