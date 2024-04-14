use crate::{logger::{log, LogType}, save::Save};

use rusqlite::Connection;

#[allow(dead_code)]
pub fn get_saves() -> Vec<Save> {
    let conn = Connection::open("Saveshare.db");

    if !conn.is_ok() {
        log(LogType::Warn("Could not connection to db".to_string()));
    }

    return vec![];

}
