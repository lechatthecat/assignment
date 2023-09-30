use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use lazy_static::lazy_static;

// Path to log file
use crate::PROJECT_PATH;

lazy_static! {
    pub static ref LOG_PATH: String = format!("{}/log/actix.log", PROJECT_PATH);
}

/// List of different types of log headers.
#[allow(dead_code)]
pub enum Header {
    SUCCESS,
    INFO,
    WARNING,
    ERROR
}

/// Logs a message to the console.
pub fn log(header: Header, message: &str) {
    // Type of message to log
    let header = match header {
        Header::SUCCESS => "SUCCESS",
        Header::INFO => "INFO",
        Header::WARNING => "WARNING",
        Header::ERROR => "ERROR"
    };

    // Print the log to the console
    println!("[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message);

    // Write the log to a file
    if Path::new(&*LOG_PATH).exists() {
        let mut log_file = OpenOptions::new().append(true).open(&*LOG_PATH).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    } else {
        let mut log_file = OpenOptions::new().create_new(true).append(true).open(&*LOG_PATH).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    }
}
