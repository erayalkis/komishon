#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod helpers;

use std::path::Path;
use notify::{recommended_watcher, Watcher};
use models::deadline::{add_deadline_to_file, remove_deadline_from_file, update_file_deadline};
use models::tag::{add_tag_to_file, remove_tag_from_file};
use models::file::{get_base_dirs, base_dirs_vec, get_children_of, walk_and_save, remove_invalid_files_from_db, search_by_name, update_favorite_status,};
use helpers::database::create_db_if_not_exists;
use crate::helpers::watcher::handle_watcher_event;

fn main() {
    let files = base_dirs_vec();
    let mut watcher = recommended_watcher(|res| {
        match res {
            Ok(event) => {
                handle_watcher_event(event);
            }
            Err(err) => {
                println!("Watcher error: {:?}", err);
            }
        }
    }).unwrap();
    for file in files {
        println!("{}", file.file_name);
        let path = Path::new(&file.path[..]);
        watcher.watch(path, notify::RecursiveMode::Recursive).unwrap();
    }
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
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
        remove_deadline_from_file,
        update_favorite_status
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
