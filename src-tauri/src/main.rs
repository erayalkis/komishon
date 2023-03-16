#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod helpers;
mod watcher;

use crate::watcher::{ GLOBAL_WATCHER, create_watcher, watch_base_dirs };
use std::sync::{Mutex, Arc};
use models::deadline::{add_deadline_to_file, remove_deadline_from_file, update_file_deadline, get_deadlines};
use models::tag::{add_tag_to_file, remove_tag_from_file};
use models::file::{get_base_dirs, get_children_of, walk_and_save, remove_invalid_files_from_db, search_by_name, update_favorite_status, get_files_by_deadline, fetch_favorited_files, fetch_single_file};
use models::notification::get_notifications;
use helpers::database::create_db_if_not_exists;
use tauri::{Manager, Window, Config};

pub static GLOBAL_WINDOW: Mutex<Option<Window>> = Mutex::new(None);
pub static GLOBAL_CONFIG: Mutex<Option<Arc<Config>>> = Mutex::new(None);

fn main() {

    tauri::Builder::default()
    .setup(|app| {

        let main_window = app.get_window("main").unwrap();
        *GLOBAL_WINDOW.lock().unwrap() = Some(main_window);
        *GLOBAL_WATCHER.lock().unwrap() = Some(create_watcher());
        *GLOBAL_CONFIG.lock().unwrap() = Some(app.config());

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
        get_files_by_deadline,
        fetch_favorited_files,
        fetch_single_file,
        get_deadlines,
        get_notifications
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
