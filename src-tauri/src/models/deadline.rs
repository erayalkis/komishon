use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Deadline {
    id: Option<i64>,
    title: String,
    date: i64,
    parent_path: String,
    parent_id: i64
}

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