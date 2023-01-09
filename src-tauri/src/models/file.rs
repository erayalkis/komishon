
use crate::{models::deadline::Deadline, add_folder_to_watcher};
use crate::models::tag::Tag;
use crate::helpers::database::get_db;

use std::{collections::HashMap, ffi::OsStr, path::Path};
use sqlite::State;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct File {
    pub id: i64,
    pub file_name: String,
    pub file_type: String,
    pub path: String,
    pub parent_path: String,
    pub is_dir: i64,
    pub is_base_dir: i64,
    pub favorited: i64,
    pub byte_size: i64,
    pub tags: Option<Vec<Tag>>,
    pub deadlines: Option<Vec<Deadline>>
}

#[tauri::command(async)]
pub fn walk_and_save(base_dir: &str) {
    let conn = get_db();
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

        if is_base_dir == 1 {
            add_folder_to_watcher(entry_path.as_path());
        }

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
pub fn get_base_dirs() -> String {
    let conn = get_db();
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
            favorited: statement.read::<i64, _>("favorited").unwrap(),
            byte_size: statement.read::<i64, _>("byte_size").unwrap(),
            tags: None,
            deadlines: None
        };
        files.push(file);
    }

    let serialized = serde_json::to_string(&files).unwrap();

    return serialized;
}

pub fn base_dirs_vec() -> Vec<File> {
    let conn = get_db();
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
            favorited: statement.read::<i64, _>("favorited").unwrap(),
            byte_size: statement.read::<i64, _>("byte_size").unwrap(),
            tags: None,
            deadlines: None
        };
        files.push(file);
    }


    return files;
}

#[tauri::command]
pub fn remove_invalid_files_from_db() {
    let conn = get_db();
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
pub fn update_favorite_status(file: File, is_fav: i64) {
    let conn = get_db();
    let query = "UPDATE FILES SET favorited = ? WHERE ID = ?";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, is_fav)).unwrap();
    statement.bind((2, file.id)).unwrap();

    // TO DO: Return errors to frontend
    match statement.next() {
        Ok(_) => {}
        
        Err(_) => {}
    }
}

#[tauri::command]
pub fn get_children_of(path: &str) -> String {
    let conn = get_db();
    let query = 
    "
    SELECT F.ID AS file_id, F.file_name AS file_name, F.file_type AS file_type, F.path AS file_path, F.parent_path AS file_parent_path, F.is_dir AS is_dir, F.is_base_dir AS is_base_dir, F.favorited as favorited, F.byte_size AS byte_size,
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
                favorited: statement.read::<i64, _>("favorited").unwrap(),
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

// Remove all the duplicate code, make the query a parameter for a different function, make the code modular
#[tauri::command]
pub fn search_by_name(input: &str) -> String {
    let conn = get_db();
    let query = 
    "
    SELECT F.ID AS file_id, F.file_name AS file_name, F.file_type AS file_type, F.path AS file_path, F.parent_path AS file_parent_path, F.is_dir AS is_dir, F.is_base_dir AS is_base_dir, F.favorited as favorited, F.byte_size AS byte_size,
    T.ID AS tag_id, T.tag_name AS tag_name, T.parent_path as tag_parent_path, T.parent_id AS tag_parent_id, T.color AS tag_color, 
    D.ID AS deadline_id, D.title AS title, D.date AS date, D.parent_path AS deadline_parent_path, D.parent_id AS deadline_parent_id
    FROM FILES F
    LEFT JOIN TAGS T ON F.ID == T.parent_id
    LEFT JOIN DEADLINES D ON F.ID == D.parent_id
    WHERE F.file_name LIKE ?
    ORDER BY F.is_dir DESC;
    ";
    let mut statement = conn.prepare(query).unwrap();
    let input_with_percentage_signs = format!("%{}%", input);
    let parsed_input = input_with_percentage_signs.as_str();
    statement.bind((1, parsed_input)).unwrap();

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
                favorited: statement.read::<i64, _>("favorited").unwrap(),
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
