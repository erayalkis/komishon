use std::{ffi::OsStr, collections::HashMap};

use crate::{GLOBAL_WINDOW, models::{file::File, deadline::Deadline, tag::Tag}};

use super::database::get_db;
use sqlite::{Statement, State};
use walkdir::WalkDir;


pub fn add_new_watched_file(base_dir: &str) {
    let conn = get_db().unwrap();
    for (idx, entry) in WalkDir::new(base_dir).into_iter().enumerate() {
        let entry = entry.unwrap();
        let entry_path_str = entry.path().to_str().unwrap();
    
        let entry_path = entry.clone().into_path();
        let entry_metadata = entry.metadata().expect("Error while reading entry metadata");
    
        let base_statement = "INSERT INTO FILES (file_name, file_type, path, parent_path, is_dir, is_base_dir, byte_size) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING *";
        let mut query = conn.prepare(base_statement).expect("Error while preparing SQL statement");

        let extension = entry_path.extension().unwrap_or(OsStr::new("File")).to_str();
        let parent_path = entry_path.parent().unwrap().to_str();
        let is_dir = entry_metadata.is_dir() as i64;
        let is_base_dir = 0;

        query.bind((1, entry.file_name().to_str())).unwrap();
        query.bind((2, extension)).unwrap();
        query.bind((3, entry_path_str)).unwrap();
        query.bind((4, parent_path)).unwrap();
        query.bind((5, is_dir)).unwrap();
        query.bind((6, is_base_dir)).unwrap();
        query.bind((7, entry_metadata.len() as i64)).unwrap();

        match query.next() {
            Ok(_) => {
                if idx == 0 {
                    let file = File {
                        id: query.read::<i64, _>("ID").unwrap(),
                        file_name: query.read::<String, _>("file_name").unwrap(),
                        file_type: query.read::<String, _>("file_type").unwrap(),
                        path: query.read::<String, _>("path").unwrap(),
                        parent_path: query.read::<String, _>("parent_path").unwrap(),
                        is_dir: query.read::<i64, _>("is_dir").unwrap(),
                        is_base_dir: query.read::<i64, _>("is_base_dir").unwrap(),
                        favorited: query.read::<i64, _>("favorited").unwrap(),
                        byte_size: query.read::<i64, _>("byte_size").unwrap(),
                        tags: None,
                        deadlines: None
                    };

                    GLOBAL_WINDOW.lock().unwrap().as_mut().unwrap().emit("file-create", file).unwrap();
                }
            }
            Err(err) => {
                println!("Error caught for {}", entry_path_str);
                println!("{}", err);
            }
        }
    }
}

pub fn get_serialized_file_string(statement: &mut Statement) -> String {
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