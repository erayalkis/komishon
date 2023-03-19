use serde::{Serialize, Deserialize};
use sqlite::State;

use crate::GLOBAL_WINDOW;
use crate::helpers::database::get_db;

/// A Notification struct meant to be used with the SQLite3 database Komishon uses.
/// Has fields for each column on the NOTIFICATIONS table.
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Notification {
  pub id: Option<i64>,
  pub title: String,
  pub body: String,
}

/// Selects all the entires from the NOTIFICATIONS table.
/// Returns a vector of Notifications as a JSON string.
#[tauri::command]
pub fn get_notifications() -> Vec<Notification> {
  let conn = get_db().unwrap();
  let query = "SELECT * FROM NOTIFICATIONS";
  let mut statement = conn.prepare(query).unwrap();

  let mut notifications: Vec<Notification> = Vec::new();
  while let Ok(State::Row) = statement.next() {
      let notification = Notification {
        id: Some(statement.read::<i64, _>("ID").unwrap()),
        title: statement.read::<String, _>("title").unwrap(),
        body: statement.read::<String, _>("body").unwrap(),
      };

      notifications.push(notification)
  }

  notifications
}

/// Inserts a Notification into the NOTIFICATIONS table.
#[tauri::command]
pub fn create_notification(title: String, body: String){
  let conn = get_db().unwrap();
  let statement = "INSERT INTO NOTIFICATIONS(title, body) VALUES (?, ?) RETURNING *";

  let mut query = conn.prepare(statement).expect("Error while preparing SQL statement");
  query.bind((1, &title[..])).unwrap();
  query.bind((2, &body[..])).unwrap();

  match query.next() {
    Ok(_) => {
      let notification = Notification {
        id: Some(query.read::<i64, _>("ID").unwrap()),
        title,
        body
      };

      GLOBAL_WINDOW.lock().unwrap().as_mut().unwrap().emit("notif-create", notification).unwrap();
    } 
    
    Err(err) => {
      println!("Error while inserting notification! {}", err);
    }
  }
}

/// Deletes a Notification from the NOTIFICATIONS table where the ID matches the `id` parameter.
#[tauri::command]
pub fn delete_notification(id: i64) {
  let conn = get_db().unwrap();
  let statement = "DELETE FROM NOTIFICATIONS WHERE ID = ? RETURNING *";
  
  let mut query = conn.prepare(statement).unwrap();
  query.bind((1, id)).unwrap();

  match query.next() {
    Ok(_) => {
      let notification = Notification {
        id: Some(id),
        title: query.read::<String, _>("title").unwrap(),
        body: query.read::<String, _>("body").unwrap()
      };

      GLOBAL_WINDOW.lock().unwrap().as_mut().unwrap().emit("notif-remove", notification).unwrap();
    }

    Err(err) => {
      println!("Error while deleting notification! {}", err);
    }
  }
}