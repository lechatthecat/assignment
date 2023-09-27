#![allow(dead_code)]
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use chrono::Local;

// Path to log file
const LOG_PATH: &str = "log/actix.log";

/// List of different types of log headers.
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
    if Path::new(LOG_PATH).exists() {
        let mut log_file = OpenOptions::new().append(true).open(LOG_PATH).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    } else {
        let mut log_file = OpenOptions::new().create_new(true).append(true).open(LOG_PATH).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    }
}
