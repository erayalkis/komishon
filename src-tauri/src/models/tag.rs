use crate::helpers::database::get_db;
use serde::{Serialize, Deserialize};

/// A Tag struct meant to be used with the SQLite3 database Komishon uses.
/// Has fields for each column on the TAGS table.
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Tag {
    pub id: Option<i64>,
    pub tag_name: String,
    pub parent_path: String,
    pub parent_id: i64,
    pub color: String
}

/// Inserts a tag into the database.
/// Returns the inserted tag as a JSON string.
#[tauri::command]
pub fn add_tag_to_file(tag: Tag) -> Result<Tag, &'static str> {
    let conn = get_db().unwrap();
    let query = "INSERT INTO TAGS(tag_name, parent_path, parent_id, color) VALUES (?, ?, ?, ?) RETURNING *";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, &tag.tag_name[..])).unwrap();
    statement.bind((2, &tag.parent_path[..])).unwrap();
    statement.bind((3, tag.parent_id)).unwrap();
    statement.bind((4, &tag.color[..])).unwrap();

    match statement.next() {
        Ok(_) => {
            let tag_with_id = Tag {
                id: Some(statement.read::<i64, _>("ID").unwrap()),
                tag_name: tag.tag_name,
                parent_path: tag.parent_path,
                parent_id: tag.parent_id,
                color: tag.color
            };

            return Ok(tag_with_id);
        }
        Err(err) => {
            println!("Error while saving tag: {}", err);
            return Err("Error while saving tag! :(");
        }
    }
}

/// Removes a tag from the database where the ID matches the id of the `tag` parameter.
#[tauri::command]
pub fn remove_tag_from_file(tag: Tag) {
    let conn = get_db().unwrap();
    let query = "DELETE FROM TAGS WHERE id == ?";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, tag.id)).unwrap();

    match statement.next() {
        Ok(_) => {}
        Err(err) => {
            println!("Error while deleting tag: {}", err);
        }
    }
}
