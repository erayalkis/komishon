use std::ffi::OsStr;

use crate::{GLOBAL_WINDOW, models::file::File};

use super::database::get_db;
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