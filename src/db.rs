use crate::{logger::warn, save::Save};

use rusqlite::Connection;

#[allow(dead_code)]
pub fn get_saves() -> Vec<Save> {
    let conn = Connection::open("Saveshare.db");

    if !conn.is_ok() {
        warn("Could not connection to db");
    }

    return vec![];
}
