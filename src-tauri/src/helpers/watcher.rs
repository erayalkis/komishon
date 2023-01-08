use std::path::PathBuf;

use notify::{Event, event::RenameMode};

use crate::{helpers::file::add_new_watched_file};

use super::database::get_db;

pub fn handle_watcher_event(event: Event) {
  let conn = get_db();
  println!("{:?}", event);
  match &event.kind {
    notify::EventKind::Any => {},
    notify::EventKind::Access(_) => {},
    notify::EventKind::Create(val) => {
      println!("{:?}", &event.paths);
      println!("{:?}", val);
      add_new_watched_file(&event.paths[0].to_str().unwrap())
    },
    notify::EventKind::Modify(val) => {
      println!("{:?}", val);
      match val {
        notify::event::ModifyKind::Any => {},
        notify::event::ModifyKind::Data(_) => {},
        notify::event::ModifyKind::Metadata(_) => {},
        notify::event::ModifyKind::Name(modifynameevent) => {
          handle_name_change_event(modifynameevent, &event.paths[0]);
        }
        notify::event::ModifyKind::Other => {}
    }
    },
    notify::EventKind::Remove(_) => {
      let query = "DELETE FROM FILES WHERE path = ?";
      let mut statement = conn.prepare(query).unwrap();
      statement.bind((1, event.paths[0].to_str().unwrap())).unwrap();

      match statement.next() {
        Ok(_) => {},
        Err(err) => {
          println!("Error while deleting file: {}", err);
        }
      }
    },
    notify::EventKind::Other => todo!(),
  }
}

pub fn handle_name_change_event(name_change_event: &RenameMode, path: &PathBuf) {
  let conn = get_db();
  match name_change_event {
    RenameMode::Any => todo!(),
    RenameMode::To => {
      let target_file = "UPDATE FILES SET path = ?, file_name = ? WHERE ID = (SELECT ID FROM FILES WHERE file_name = \"WILL_UPDATE\")";
      let filename = path.file_name().unwrap().to_str().unwrap();

      let mut statement = conn.prepare(target_file).unwrap();
      statement.bind((1, path.to_str().unwrap())).unwrap();
      statement.bind((2, filename)).unwrap();

      match statement.next() {
        Ok(_) => {},
        Err(err) => {
          println!("Error while renaming file from To call: {}", err);
        }
      }
    },
    RenameMode::From => {
      println!("Received update from call for: {}", path.to_str().unwrap());
      let query = "UPDATE FILES SET file_name = \"WILL_UPDATE\" WHERE path = ?";
      let mut statement = conn.prepare(query).unwrap();
      statement.bind((1, path.to_str().unwrap())).unwrap();

      match statement.next() {
        Ok(_) => {},
        Err(err) => {
          println!("Error while renaming file temporarily: {}", err);
        }
      }
    },
    RenameMode::Both => {
      println!("both!")
    },
    RenameMode::Other => todo!(),
}
}