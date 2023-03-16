
use crate::helpers::watcher::handle_watcher_event;
use crate::models::file::base_dirs_vec;

use std::path::Path;
use notify::recommended_watcher;

#[cfg(target_os = "windows")]
use notify::ReadDirectoryChangesWatcher;
#[cfg(target_os = "linux")]
use notify::Watcher;

use std::sync::Mutex;

#[cfg(target_os = "windows")]
pub static GLOBAL_WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);

#[cfg(target_os = "linux")]
pub static GLOBAL_WATCHER: Mutex<Option<notify::INotifyWatcher>> = Mutex::new(None);


#[cfg(target_os = "windows")]
pub fn create_watcher() -> ReadDirectoryChangesWatcher {
    // Make this mutable again if errors start popping up (dont think its necessary tho)
    let file_watcher = recommended_watcher(|res| {
        match res {
            Ok(event) => {
                handle_watcher_event(event);
            }
            Err(err) => {
                println!("Watcher error: {:?}", err);
            }
        }
    }).unwrap();

    return file_watcher;
}

#[cfg(target_os = "linux")]
pub fn create_watcher() -> notify::INotifyWatcher {
    // Make this mutable again if errors start popping up (dont think its necessary tho)
    let file_watcher = recommended_watcher(|res| {
        match res {
            Ok(event) => {
                handle_watcher_event(event);
            }
            Err(err) => {
                println!("Watcher error: {:?}", err);
            }
        }
    }).unwrap();

    return file_watcher;
}

#[tauri::command]
pub fn watch_base_dirs() {
    let files = base_dirs_vec();
    let mut binding = GLOBAL_WATCHER.lock().unwrap();
    let watcher = binding.as_mut().unwrap();

    for file in files {
        println!("{}", file.file_name);
        let path = Path::new(&file.path[..]);
        watcher.watch(path, notify::RecursiveMode::Recursive).unwrap();
    }
}

pub fn add_folder_to_watcher(path: &Path) {
    let mut mutex_guard = GLOBAL_WATCHER.lock().unwrap();
    let watcher = mutex_guard.as_mut().unwrap();

    watcher.watch(path, notify::RecursiveMode::Recursive).unwrap();
}
