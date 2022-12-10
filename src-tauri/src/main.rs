#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, ffi::OsStr};

use walkdir::WalkDir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_db_if_not_exists() {
    let conn = sqlite::open("entries.db").expect("Error while accessing database");
    let query = "
        CREATE TABLE IF NOT EXISTS FILES (file_name TEXT, file_type TEXT, path TEXT, parent_path TEXT, is_dir INTEGER, byte_size INTEGER)
    ";
    conn.execute(query).expect("Error while executing SQL statement");
}

#[tauri::command]
fn return_map() -> HashMap<String, i32> {
    let conn = sqlite::open("entries.db");
    let mut test_hashmap = HashMap::new();
    test_hashmap.insert(String::from("Blue"), 0);
    test_hashmap.insert(String::from("Green"), 1);
    test_hashmap.insert(String::from("Red"), 2);

    return test_hashmap;
}

#[tauri::command]
fn walk_and_save(base_dir: &str) {
    create_db_if_not_exists();

    let conn = sqlite::open("entries.db").expect("Error while accessing database");
    for entry in WalkDir::new(base_dir) {
        let entry = entry.unwrap();
        println!("{}", entry.path().to_str().unwrap());
    
        let entry_path = entry.clone().into_path();
        let entry_metadata = entry.metadata().expect("Error while reading entry metadata");
    
        let base_statement = "INSERT INTO FILES (file_name, file_type, path, parent_path, is_dir, byte_size) VALUES (?, ?, ?, ?, ?, ?)";
        let mut query = conn.prepare(base_statement).expect("Error while preparing SQL statement");

        let path = entry_path.extension().unwrap_or(OsStr::new("File")).to_str();
        let parent_path = entry_path.parent().expect("Error while raeding parent path").to_str();
        let is_dir = entry_metadata.is_dir() as i64;

        query.bind((1, entry.file_name().to_str())).unwrap();
        query.bind((2, path)).unwrap();
        query.bind((3, entry.path().to_str())).unwrap();
        query.bind((4, parent_path)).unwrap();
        query.bind((5, is_dir)).unwrap();
        query.bind((6, entry_metadata.len() as i64)).unwrap();

        query.next().expect("Error while executing SQL INSERT");
    }
}

#[tauri::command]
fn read_all_from_db() {
    let conn = sqlite::open("entries.db").unwrap();
    let query = "SELECT * FROM FILES;";
    
    conn
    .iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        true
    })
    .unwrap();
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, return_map, walk_and_save, read_all_from_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
