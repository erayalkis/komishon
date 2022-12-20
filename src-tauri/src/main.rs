#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, ffi::OsStr, path::Path};

use serde::{Serialize, Deserialize};
use sqlite::{State};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct File {
    id: i64,
    file_name: String,
    file_type: String,
    path: String,
    parent_path: String,
    is_dir: i64,
    is_base_dir: i64,
    byte_size: i64,
    tags: Option<Vec<Tag>>,
    deadlines: Option<Vec<Deadline>>
}

#[derive(Serialize, Deserialize)]
struct Tag {
    id: Option<i64>,
    tag_name: String,
    parent_path: String,
    parent_id: i64,
    color: String
}

#[derive(Serialize, Deserialize)]
struct Deadline {
    id: Option<i64>,
    title: String,
    date: i64,
    parent_path: String,
    parent_id: i64
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_db_if_not_exists(to: &str) {
    let conn = sqlite::open(to).expect("Error while accessing database");
    let query = "
        PRAGMA journal_mode=WAL;
        PRAGMA foreign_keys=ON;
        CREATE TABLE IF NOT EXISTS FILES (ID INTEGER PRIMARY KEY AUTOINCREMENT, file_name TEXT, file_type TEXT, path TEXT, parent_path TEXT, is_dir INTEGER, is_base_dir INTEGER, byte_size INTEGER);
        CREATE TABLE IF NOT EXISTS TAGS (ID INTEGER PRIMARY KEY AUTOINCREMENT, tag_name TEXT, parent_path TEXT, parent_id INTEGER, color TEXT, FOREIGN KEY(parent_id) REFERENCES TAGS(ID));
        CREATE TABLE IF NOT EXISTS DEADLINES (ID INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, date DATETIME, parent_path TEXT, parent_id INTEGER, FOREIGN KEY(parent_id) REFERENCES TAGS(ID));
        CREATE UNIQUE INDEX IF NOT EXISTS unique_file_index ON FILES (file_type, path);
        CREATE UNIQUE INDEX IF NOT EXISTS unique_tag_index ON TAGS (parent_id, tag_name);
        CREATE INDEX IF NOT EXISTS path_index ON FILES (path);
        CREATE INDEX IF NOT EXISTS parent_path_index ON FILES (parent_path);
    ";
    conn.execute(query).expect("Error while executing SQL statement");
}

#[tauri::command]
fn return_map() -> HashMap<String, i32> {
    let mut test_hashmap = HashMap::new();
    test_hashmap.insert(String::from("Blue"), 0);
    test_hashmap.insert(String::from("Green"), 1);
    test_hashmap.insert(String::from("Red"), 2);

    return test_hashmap;
}

#[tauri::command(async)]
fn walk_and_save(base_dir: &str, to: &str) {
    let conn = sqlite::open(to).expect("Error while accessing database");
    for (idx, entry) in WalkDir::new(base_dir).into_iter().enumerate() {
        let entry = entry.unwrap();
        let entry_path_str = entry.path().to_str().unwrap();
    
        let entry_path = entry.clone().into_path();
        let entry_metadata = entry.metadata().expect("Error while reading entry metadata");
    
        let base_statement = "INSERT INTO FILES (file_name, file_type, path, parent_path, is_dir, is_base_dir, byte_size) VALUES (?, ?, ?, ?, ?, ?, ?);";
        let mut query = conn.prepare(base_statement).expect("Error while preparing SQL statement");

        let extension = entry_path.extension().unwrap_or(OsStr::new("File")).to_str();
        let parent_path = entry_path.parent().unwrap().to_str();
        let is_dir = entry_metadata.is_dir() as i64;
        let is_base_dir = (idx == 0) as i64;

        query.bind((1, entry.file_name().to_str())).unwrap();
        query.bind((2, extension)).unwrap();
        query.bind((3, entry_path_str)).unwrap();
        query.bind((4, parent_path)).unwrap();
        query.bind((5, is_dir)).unwrap();
        query.bind((6, is_base_dir)).unwrap();
        query.bind((7, entry_metadata.len() as i64)).unwrap();

        match query.next() {
            Ok(_) => {
            }
            Err(err) => {
                println!("Error caught for {}", entry_path_str);
                println!("{}", err);
            }
        }
    }
}

#[tauri::command]
fn remove_invalid_files_from_db(db_path: &str) {
    let conn = sqlite::open(db_path).unwrap();
    let query = "SELECT * FROM FILES;";
    let mut statement = conn.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        let file_path_string = statement.read::<String, _>("path").unwrap();
        let copy = file_path_string.clone();

        let path = Path::new(&copy);

        if path.exists() == false {
            let delete_query = "DELETE FROM FILES WHERE path = ?";
            let mut query = conn.prepare(delete_query).unwrap();

            query.bind((1, &file_path_string[..])).unwrap();

            match query.next() {
                Ok(_) => {
                }
                Err(err) => {
                    println!("Error while deleting {}", file_path_string);
                    println!("{}", err);
                }
            } 
        }
    }
}

#[tauri::command]
fn read_all_from_db(db_path: &str) {
    let conn = sqlite::open(db_path).unwrap();
    let query = "SELECT * FROM FILES;";
    
    conn
    .iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        println!("-----------------");
        true
    })
    .unwrap();
}

#[tauri::command]
fn get_base_dirs(db_path: &str) -> String {
    let conn = sqlite::open(db_path).unwrap();
    let query = "SELECT * FROM FILES WHERE is_base_dir == 1";
    let mut statement = conn.prepare(query).unwrap();

    let mut files: Vec<File> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let file = File {
            id: statement.read::<i64, _>("ID").unwrap(),
            file_name: statement.read::<String, _>("file_name").unwrap(),
            file_type: statement.read::<String, _>("file_type").unwrap(),
            path: statement.read::<String, _>("path").unwrap(),
            parent_path: statement.read::<String, _>("parent_path").unwrap(),
            is_dir: statement.read::<i64, _>("is_dir").unwrap(),
            is_base_dir: statement.read::<i64, _>("is_base_dir").unwrap(),
            byte_size: statement.read::<i64, _>("byte_size").unwrap(),
            tags: None,
            deadlines: None
        };
        files.push(file);
    }

    let serialized = serde_json::to_string(&files).unwrap();

    return serialized;
}

#[tauri::command]
fn get_children_of(db_path: &str, path: &str) -> String {
    let conn = sqlite::open(db_path).unwrap();
    let query = 
    "
    SELECT F.ID AS file_id, F.file_name AS file_name, F.file_type AS file_type, F.path AS file_path, F.parent_path AS file_parent_path, F.is_dir AS is_dir, F.is_base_dir AS is_base_dir, F.byte_size AS byte_size,
    T.ID AS tag_id, T.tag_name AS tag_name, T.parent_path as tag_parent_path, T.parent_id AS tag_parent_id, T.color AS tag_color, 
    D.ID AS deadline_id, D.title AS title, D.date AS date, D.parent_path AS deadline_parent_path, D.parent_id AS deadline_parent_id
    FROM FILES F
    LEFT JOIN TAGS T ON F.ID == T.parent_id
    LEFT JOIN DEADLINES D ON F.ID == D.parent_id
    WHERE F.parent_path = ?
    ORDER BY F.is_dir DESC;
    ";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, path)).unwrap();

    let mut files: Vec<File> = Vec::new();
    let mut seen: HashMap<String, bool> = HashMap::new();
    let mut last_seen_file_id: i64 = 0;

    while let Ok(State::Row) = statement.next() {
        let file_id = statement.read::<i64, _>("file_id").unwrap();
        let same_file_as_last = file_id == last_seen_file_id;

        if !same_file_as_last {
            let file = File {
                id: file_id,
                file_name: statement.read::<String, _>("file_name").unwrap(),
                file_type: statement.read::<String, _>("file_type").unwrap(),
                path: statement.read::<String, _>("file_path").unwrap(),
                parent_path: statement.read::<String, _>("file_parent_path").unwrap(),
                is_dir: statement.read::<i64, _>("is_dir").unwrap(),
                is_base_dir: statement.read::<i64, _>("is_base_dir").unwrap(),
                byte_size: statement.read::<i64, _>("byte_size").unwrap(),
                tags: Some(Vec::new()),
                deadlines: Some(Vec::new())
            };

            files.push(file);
        }

        let last_file = files.last_mut().unwrap();
        match statement.read::<String, _>("tag_name") {
            Ok(val) => {
                println!("Got something: {}", val);
                let tag_id = statement.read::<i64, _>("tag_id").unwrap();
                let unique_tag_id = format!("{}-{}", tag_id, val);

                if !seen.contains_key(&unique_tag_id) {
                    let tag = Tag {
                        id: Some(tag_id),
                        tag_name: val,
                        parent_path: statement.read::<String, _>("tag_parent_path").unwrap(),
                        parent_id: statement.read::<i64, _>("tag_parent_id").unwrap(),
                        color: statement.read::<String, _>("tag_color").unwrap(),
                    };

                    last_file.tags.as_mut().unwrap().push(tag);
                    seen.insert(unique_tag_id, true);
                }
            }
            Err(_) => {}
        }

        match statement.read::<String, _>("title") {
            Ok(val) => {
                let deadline_id = statement.read::<i64, _>("deadline_id").unwrap();
                let unique_deadline_id = format!("{}-{}", deadline_id, val);

                if !seen.contains_key(&unique_deadline_id) {
                    let deadline = Deadline {
                        id: Some(deadline_id),
                        title: val,
                        date: statement.read::<i64, _>("date").unwrap(),
                        parent_path: statement.read::<String, _>("deadline_parent_path").unwrap(),
                        parent_id: statement.read::<i64, _>("deadline_parent_id").unwrap(),
                    };

                    last_file.deadlines.as_mut().unwrap().push(deadline);
                    seen.insert(unique_deadline_id, true);
                }
            }
            Err(_) => {}
        }

        last_seen_file_id = file_id;
    }

    let serialized = serde_json::to_string(&files).unwrap();

    return serialized;
}

#[tauri::command]
fn add_tag_to_file(db_path: &str, tag: Tag) {
    let conn = sqlite::open(db_path).unwrap();
    let query = "INSERT INTO TAGS(tag_name, parent_path, parent_id, color) VALUES (?, ?, ?, ?)";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, &tag.tag_name[..])).unwrap();
    statement.bind((2, &tag.parent_path[..])).unwrap();
    statement.bind((3, tag.parent_id)).unwrap();
    statement.bind((4, &tag.color[..])).unwrap();

    match statement.next() {
        Ok(_) => { 
            println!("Added tag");
        }
        Err(err) => {
            println!("Error while saving tag: {}", err);
        }
    }
}

#[tauri::command]
fn remove_tag_from_file(db_path: &str, tag: Tag) {
    let conn = sqlite::open(db_path).unwrap();
    let query = "DELETE FROM TAGS WHERE id == ?";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, tag.id)).unwrap();

    match statement.next() {
        Ok(_) => {}
        Err(err) => {
            println!("Error while deleting tag: {}", err);
        }
    }
}

// #[tauri::command]
// fn update_file_tag() {

// }

#[tauri::command]
fn add_deadline_to_file(db_path: &str, deadline: Deadline) {
    let conn = sqlite::open(db_path).unwrap();
    let query = "INSERT INTO DEADLINES(title, date, parent_path, parent_id) VALUES (?, ?, ?, ?)";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, &deadline.title[..])).unwrap();
    statement.bind((2, deadline.date)).unwrap();
    statement.bind((3, &deadline.parent_path[..])).unwrap();
    statement.bind((4, deadline.parent_id)).unwrap();

    match statement.next() {
        Ok(_) => {}
        Err(err) => {
            println!("Error while deleting tag: {}", err);
        }
    }
}

#[tauri::command]
fn remove_deadline_from_file(db_path: &str, deadline: Deadline) {
    let conn = sqlite::open(db_path).unwrap();
    let query = "DELETE FROM DEADLINES WHERE id = ?";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, deadline.id)).unwrap();

    match statement.next() {
        Ok(_) => {}
        Err(err) => {
            println!("Error while deleting deadline: {}", err);
        }
    }
}

#[tauri::command]
fn update_file_deadline(db_path: &str, deadline: Deadline) {
    let conn = sqlite::open(db_path).unwrap();
    let query = "UPDATE DEADLINES SET title = ?, date = ? WHERE id = ?";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, &deadline.title[..])).unwrap();
    statement.bind((2, deadline.date)).unwrap();
    statement.bind((3, deadline.id)).unwrap();

    match statement.next() {
        Ok(_) => {}
        Err(err) => {
            println!("Error while deleting deadline: {}", err);
        }
    }
}

fn main() {
    println!("{}", tauri::api::path::data_dir().unwrap().display());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet, 
            return_map, 
            walk_and_save, 
            read_all_from_db, 
            remove_invalid_files_from_db, 
            get_children_of, 
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
