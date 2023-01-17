use crate::helpers::database::get_db;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Deadline {
    pub id: Option<i64>,
    pub title: String,
    pub date: i64,
    pub parent_path: String,
    pub parent_id: i64
}

#[tauri::command]
pub fn add_deadline_to_file(deadline: Deadline) -> Result<Deadline, &'static str> {
    let conn = get_db().unwrap();
    let query = "INSERT INTO DEADLINES(title, date, parent_path, parent_id) VALUES (?, ?, ?, ?) RETURNING *";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, &deadline.title[..])).unwrap();
    statement.bind((2, deadline.date)).unwrap();
    statement.bind((3, &deadline.parent_path[..])).unwrap();
    statement.bind((4, deadline.parent_id)).unwrap();

    match statement.next() {
        Ok(_) => {
            let deadline_with_id = Deadline {
                id: Some(statement.read::<i64, _>("ID").unwrap()),
                title: deadline.title,
                date: deadline.date,
                parent_id: deadline.parent_id,
                parent_path: deadline.parent_path
            };

            return Ok(deadline_with_id);
        }
        Err(err) => {
            println!("Error while saving deadline: {}", err);
            return Err("Error while saving deadlines! :(");
        }
    }
}

#[tauri::command]
pub fn remove_deadline_from_file(deadline: Deadline) {
    let conn = get_db().unwrap();
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
pub fn update_file_deadline(deadline: Deadline) {
    let conn = get_db().unwrap();
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
