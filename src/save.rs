use std::str::FromStr;

use uuid::Uuid;

#[allow(dead_code)]
pub struct Save {
    pub id: Uuid,
    pub xml: String,
    pub game_file: String,
    pub name: String,
    pub world_id: u64,
    pub current_host_id: u64,
    pub version: i32
}

#[allow(dead_code)]
impl Save {
    pub fn from(id: &str, xml: String, game_file: String, name: String, world_id: u64, current_host_id: u64, version: i32) -> Self {
        Save {
            id: Uuid::from_str(id).expect("Invalid Uuid"),
            xml,
            game_file,
            name,
            world_id,
            current_host_id,
            version
        }
    }
}
