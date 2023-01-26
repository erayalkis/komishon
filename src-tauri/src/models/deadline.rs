use crate::helpers::database::get_db;
use serde::{Serialize, Deserialize};
use sqlite::State;

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
pub fn get_deadlines() -> Result<String, &'static str> {
    let conn = get_db().unwrap();
    let query = "SELECT * FROM DEADLINES;";
    let mut statement = conn.prepare(query).unwrap();
    
    let mut deadlines: Vec<Deadline> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let deadline = Deadline {
            id: Some(statement.read::<i64, _>("ID").unwrap()),
            title: statement.read::<String, _>("title").unwrap(),
            date: statement.read::<i64, _>("date").unwrap(),
            parent_path: statement.read::<String, _>("parent_path").unwrap(),
            parent_id: statement.read::<i64,_>("parent_id").unwrap()
        };

        deadlines.push(deadline);
    }

    let serialized = serde_json::to_string(&deadlines).unwrap();
    return Ok(serialized)
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
