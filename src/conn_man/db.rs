use crate::{logging::logger::warn, state::save::Save};

use rusqlite::Connection;

#[allow(dead_code)]
pub fn get_saves() -> Option<Vec<Save>> {
    let conn = Connection::open("Saveshare.db");

    if !conn.is_ok() {
        warn("Could not connect to db");
        return None;
    }

    Some(vec![])
}

#[allow(dead_code)]
pub fn save() -> Option<()> {

    let conn = Connection::open("Saveshare.db");

    if !conn.is_ok() {
        warn("Could not connect to db");
        return None;
    }

    Some(())
}
