use std::fs;

use rusqlite::Connection;

use refinery::embed_migrations;

pub fn get_library_db() -> Option<Connection> {
    if cfg!(test) {
        return Connection::open_in_memory().ok();
    }

    let dirs = crate::util::get_project_dirs();

    let config_dir = dirs.config_dir();

    fs::create_dir_all(config_dir).ok()?;

    let db_path = config_dir.join("database.sqlite");

    Connection::open(db_path).ok()
}

pub fn run_migrations(conn: &mut Connection) {
    self::migrations::runner().run(conn).unwrap();
}

embed_migrations!("./migrations");
