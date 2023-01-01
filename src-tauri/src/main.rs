#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod helpers;
use models::deadline::{add_deadline_to_file, remove_deadline_from_file, update_file_deadline};
use models::tag::{add_tag_to_file, remove_tag_from_file};
use models::file::{get_base_dirs, get_children_of, walk_and_save, remove_invalid_files_from_db, search_by_name};
use helpers::database::create_db_if_not_exists;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet, 
            walk_and_save, 
            remove_invalid_files_from_db, 
            get_children_of,
            search_by_name,
            get_base_dirs, 
            create_db_if_not_exists,
            add_tag_to_file,
            remove_tag_from_file,
            add_deadline_to_file,
            update_file_deadline,
            remove_deadline_from_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
