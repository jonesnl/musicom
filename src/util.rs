use std::path::PathBuf;
use std::fs;

use directories::ProjectDirs;

pub type NotifierCb = Box<dyn Fn() + Send + Sync + 'static>;

#[derive(Default)]
pub struct Notifier {
    subscribers: Vec<NotifierCb>,
}

impl Notifier {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn register(&mut self, cb: NotifierCb) {
        self.subscribers.push(cb);
    }

    pub fn notify(&self) {
        self.subscribers.iter().for_each(|cb| cb());
    }
}

fn get_project_dirs() -> ProjectDirs {
    ProjectDirs::from("com.jonesnl", "Nate Jones", "Musicom").unwrap()
}

pub fn get_database_path() -> PathBuf {
    let dirs = get_project_dirs();

    let config_dir = dirs.config_dir();

    fs::create_dir_all(config_dir).unwrap();

    config_dir.join("database.sqlite")
}
