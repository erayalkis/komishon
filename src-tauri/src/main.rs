#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod helpers;

use std::path::Path;
use std::sync::Mutex;
use notify::{recommended_watcher, Watcher, ReadDirectoryChangesWatcher};
use models::deadline::{add_deadline_to_file, remove_deadline_from_file, update_file_deadline};
use models::tag::{add_tag_to_file, remove_tag_from_file};
use models::file::{get_base_dirs, base_dirs_vec, get_children_of, walk_and_save, remove_invalid_files_from_db, search_by_name, update_favorite_status};
use helpers::database::create_db_if_not_exists;
use crate::helpers::watcher::handle_watcher_event;


pub static GLOBAL_WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None); 

fn create_watcher() -> ReadDirectoryChangesWatcher {
    // Make this mutable again if errors start popping up (dont think its necessary tho)
    let file_watcher = recommended_watcher(|res| {
        match res {
            Ok(event) => {
                handle_watcher_event(event);
            }
            Err(err) => {
                println!("Watcher error: {:?}", err);
            }
        }
    }).unwrap();

    return file_watcher;
}

#[tauri::command]
fn watch_base_dirs() {
    let files = base_dirs_vec();
    let mut binding = GLOBAL_WATCHER.lock().unwrap();
    let watcher = binding.as_mut().unwrap();

    for file in files {
        println!("{}", file.file_name);
        let path = Path::new(&file.path[..]);
        watcher.watch(path, notify::RecursiveMode::Recursive).unwrap();
    }
}

fn main() {
    *GLOBAL_WATCHER.lock().unwrap() = Some(create_watcher());

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
        update_favorite_status,
        watch_base_dirs
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
