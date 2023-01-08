use notify::{Event, event::RenameMode};

use crate::models::file::walk_and_save;

pub fn handle_watcher_event(event: Event) {
  println!("{:?}", event);
  match &event.kind {
    notify::EventKind::Any => {},
    notify::EventKind::Access(_) => {},
    notify::EventKind::Create(val) => {
      println!("{:?}", &event.paths);
      println!("{:?}", val);
      walk_and_save(&event.paths[0].to_str().unwrap());
    },
    notify::EventKind::Modify(val) => {
      println!("{:?}", val);
      match val {
        notify::event::ModifyKind::Any => {},
        notify::event::ModifyKind::Data(_) => {},
        notify::event::ModifyKind::Metadata(_) => {},
        notify::event::ModifyKind::Name(modifynameevent) => {
          handle_name_change_event(modifynameevent);
        }
        notify::event::ModifyKind::Other => {}
    }
    },
    notify::EventKind::Remove(val) => {
      println!("{:?}", val);
    },
    notify::EventKind::Other => todo!(),
  }
}

pub fn handle_name_change_event(name_change_event: &RenameMode) {
  match name_change_event {
    RenameMode::Any => todo!(),
    RenameMode::To => {
      println!("To!");
    },
    RenameMode::From => {
      println!("From!");
    },
    RenameMode::Both => {
      println!("both!")
    },
    RenameMode::Other => todo!(),
}
}