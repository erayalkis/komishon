use notify::Event;

use crate::models::file::walk_and_save;

pub fn handle_watcher_event(event: Event) {
  println!("{:?}", event);
  match &event.kind {
    notify::EventKind::Any => todo!(),
    notify::EventKind::Access(_) => todo!(),
    notify::EventKind::Create(val) => {
      println!("{:?}", &event.paths);
      println!("{:?}", val);
      walk_and_save(&event.paths[0].to_str().unwrap());
    },
    notify::EventKind::Modify(val) => {
      println!("{:?}", val);
    },
    notify::EventKind::Remove(val) => {
      println!("{:?}", val);
    },
    notify::EventKind::Other => todo!(),
}
}