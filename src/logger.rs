use crate::LogType::{Info, Panic, Warn};

use colored::Colorize;

pub enum LogType {
    Info(String),
    Warn(String),
    Panic,
}

pub fn log(log: LogType) {
    match log {
        Info(s) => println!("--- {} ---", s),
        Warn(s) => println!("--- {} ---", s.red()),
        Panic => println!("Panicing!")
    }
}

pub fn warn(msg: &str) {
    log(LogType::Warn(msg.to_string()))
}

pub fn info(msg: &str) {
    log(LogType::Info(msg.to_string()))
}
