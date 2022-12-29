use std::path::PathBuf;
use sqlite::Connection;

pub fn database_path() -> PathBuf {
  let conf = tauri::Config::default();
  let appdata_path = tauri::api::path::app_data_dir(&conf).unwrap();
  println!("appdata_path: {}", appdata_path.display());
  let db_path = appdata_path.join("com.tauri.dev").join("entries.db");

  return db_path;
}

pub fn get_db() -> Connection {
  let db_path = database_path().display().to_string();
  println!("{}", db_path);
  let conn = sqlite::open(db_path).unwrap();
  return conn;
}

#[tauri::command]
pub fn create_db_if_not_exists() {
    let conn = get_db();
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