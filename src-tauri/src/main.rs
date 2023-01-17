#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod helpers;

use std::path::Path;
use std::sync::Mutex;
use notify::recommended_watcher;
#[cfg(target_os="macos")]
use notify::WatcherKind::ReadDirectoryChangesWatcher;
#[cfg(target_os="windows")]
use notify::ReadDirectoryChangesWatcher;
use notify::Watcher;
use models::deadline::{add_deadline_to_file, remove_deadline_from_file, update_file_deadline};
use models::tag::{add_tag_to_file, remove_tag_from_file};
use models::file::{get_base_dirs, base_dirs_vec, get_children_of, walk_and_save, remove_invalid_files_from_db, search_by_name, update_favorite_status, fetch_files_with_deadlines, fetch_favorited_files, fetch_single_file};
use helpers::database::create_db_if_not_exists;
use tauri::{Manager, Window};
use crate::helpers::watcher::handle_watcher_event;

pub static GLOBAL_WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);
pub static GLOBAL_WINDOW: Mutex<Option<Window>> = Mutex::new(None);

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

fn add_folder_to_watcher(path: &Path) {
    let mut mutex_guard = GLOBAL_WATCHER.lock().unwrap();
    let watcher = mutex_guard.as_mut().unwrap();

    watcher.watch(path, notify::RecursiveMode::Recursive).unwrap();
}

fn main() {

    tauri::Builder::default()
    .setup(|app| {

        let main_window = app.get_window("main").unwrap();
        *GLOBAL_WINDOW.lock().unwrap() = Some(main_window);
        *GLOBAL_WATCHER.lock().unwrap() = Some(create_watcher());

        Ok(())
    })
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
        watch_base_dirs,
        fetch_files_with_deadlines,
        fetch_favorited_files,
        fetch_single_file
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
