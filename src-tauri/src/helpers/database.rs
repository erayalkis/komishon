use std::{path::PathBuf};
use sqlite::{Connection, Statement};
use std::sync::Mutex;

use crate::GLOBAL_CONFIG;

pub static DB: Mutex<Option<Connection>> = Mutex::new(None);

pub fn database_path() -> PathBuf {
  let mutex_binding = GLOBAL_CONFIG.lock().unwrap();
  let conf = mutex_binding.as_ref().unwrap();

  let appdata_path = tauri::api::path::app_data_dir(&conf).unwrap();
  let db_path = appdata_path.join("entries.db");

  return db_path;
}

pub fn get_db() -> std::option::Option<Connection> {
  if DB.lock().unwrap().is_none() {
    connect_to_db();
  }

  return DB.lock().unwrap().take();
}

fn connect_to_db() {
  let db_path = database_path().display().to_string();
  let conn = sqlite::open(db_path).unwrap();
  *DB.lock().unwrap() = Some(conn);
}

pub fn get_statement_from_query<'a, T>(conn: &'a Connection, query: &'a str, bindings: Vec<(usize, T)>) -> Statement<'a> where T: sqlite::BindableWithIndex{
  let mut statement = conn.prepare(query).unwrap();

  for binding in bindings {
    statement.bind(binding).unwrap();
  }

  return statement;
}

#[tauri::command]
pub fn create_db_if_not_exists() {
    let conn = get_db().unwrap();
    let query = "
        PRAGMA journal_mode=WAL;
        PRAGMA foreign_keys=ON;
        CREATE TABLE IF NOT EXISTS FILES (ID INTEGER PRIMARY KEY AUTOINCREMENT, file_name TEXT, file_type TEXT, path TEXT, parent_path TEXT, is_dir INTEGER, is_base_dir INTEGER, favorited INTEGER, byte_size INTEGER);
        CREATE TABLE IF NOT EXISTS TAGS (ID INTEGER PRIMARY KEY AUTOINCREMENT, tag_name TEXT, parent_path TEXT, parent_id INTEGER, color TEXT, FOREIGN KEY(parent_id) REFERENCES TAGS(ID));
        CREATE TABLE IF NOT EXISTS DEADLINES (ID INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, date DATETIME, parent_path TEXT, parent_id INTEGER, FOREIGN KEY(parent_id) REFERENCES TAGS(ID));
        CREATE TABLE IF NOT EXISTS NOTIFICATIONS (ID INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, body TEXT);
        CREATE UNIQUE INDEX IF NOT EXISTS unique_file_index ON FILES (file_type, path);
        CREATE UNIQUE INDEX IF NOT EXISTS unique_tag_index ON TAGS (parent_id, tag_name);
        CREATE INDEX IF NOT EXISTS path_index ON FILES (path);
        CREATE INDEX IF NOT EXISTS parent_path_index ON FILES (parent_path);
    ";
    conn.execute(query).expect("Error while executing SQL statement");
}