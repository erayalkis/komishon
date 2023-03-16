use serde::{Serialize, Deserialize};
use sqlite::Error;

use crate::helpers::database::get_db;

#[derive(Serialize, Deserialize)]
pub struct Notification {
  pub id: Option<i64>,
  pub title: String,
  pub body: String,
}

pub fn create_notification(title: String, body: String) -> Result<Notification, Error> {
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

      Ok(notification)
    } 
    
    Err(err) => {
      println!("Error while inserting notification! {}", err);
      Err(err)
    }
  }
}

pub fn delete_notification(id: i64) -> Result<Notification, Error> {
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

      Ok(notification)
    }

    Err(err) => {
      println!("Error while deleting notification! {}", err);
      Err(err)
    }
  }
}