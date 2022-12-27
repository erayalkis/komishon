use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub tag_name: String,
    pub parent_path: String,
    pub parent_id: i64,
    pub color: String
}

#[tauri::command]
pub fn add_tag_to_file(db_path: &str, tag: Tag) {
    let conn = sqlite::open(db_path).unwrap();
    let query = "INSERT INTO TAGS(tag_name, parent_path, parent_id, color) VALUES (?, ?, ?, ?)";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, &tag.tag_name[..])).unwrap();
    statement.bind((2, &tag.parent_path[..])).unwrap();
    statement.bind((3, tag.parent_id)).unwrap();
    statement.bind((4, &tag.color[..])).unwrap();

    match statement.next() {
        Ok(_) => { 
            println!("Added tag");
        }
        Err(err) => {
            println!("Error while saving tag: {}", err);
        }
    }
}

#[tauri::command]
pub fn remove_tag_from_file(db_path: &str, tag: Tag) {
    let conn = sqlite::open(db_path).unwrap();
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
